// This file implements a futures-compatible executor which schedules futures
// onto the JavaScript event loop.
//
// TODO: Verify that this works correctly for multiple threads.

use futures_core::{Future, Async, Never};
use futures_core::executor::{Executor, SpawnError};
use futures_core::task::{LocalMap, Wake, Waker, Context};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::collections::VecDeque;
use std::cmp;
use webcore::try_from::TryInto;
use webcore::value::Reference;


// TODO: Determine optimal values for these constants
// Initial capacity of the event queue
const INITIAL_QUEUE_CAPACITY: usize = 10;

// Iterations to wait before allowing the queue to shrink
const QUEUE_SHRINK_DELAY: usize = 10;


type BoxedFuture = Box< Future< Item = (), Error = Never > + 'static + Send >;

struct TaskInner {
    map: LocalMap,
    future: Option< BoxedFuture >,
}

impl ::std::fmt::Debug for TaskInner {
    fn fmt( &self, fmt: &mut ::std::fmt::Formatter ) -> Result< (), ::std::fmt::Error > {
        fmt.debug_struct( "TaskInner" )
            .field( "map", &self.map )
            .field( "future", &self.future.as_ref().map( |_| "BoxedFuture" ) )
            .finish()
    }
}


// TODO is it possible to avoid the Mutex ?
#[derive(Debug)]
struct Task {
    is_queued: AtomicBool,
    inner: Mutex< TaskInner >,
}

impl Task {
    fn new( future: BoxedFuture ) -> Arc< Self > {
        Arc::new( Self {
            is_queued: AtomicBool::new( false ),
            inner: Mutex::new( TaskInner {
                map: LocalMap::new(),
                future: Some( future ),
            } ),
        } )
    }

    fn poll( arc: Arc< Self > ) {
        let mut lock = arc.inner.lock().unwrap();

        // Take the future so that if we panic it gets dropped
        if let Some( mut future ) = lock.future.take() {
            // Clear `is_queued` flag so that it will re-queue if poll calls waker.wake()
            arc.is_queued.store( false, Ordering::SeqCst );

            let poll = {
                // TODO is there some way of saving these so they don't need to be recreated all the time ?
                let waker = Waker::from( arc.clone() );

                let mut executor = &EVENT_LOOP;

                let mut cx = Context::new( &mut lock.map, &waker, &mut executor );

                future.poll( &mut cx )
            };

            if let Ok( Async::Pending ) = poll {
                // Future was not ready, so put it back
                lock.future = Some( future );
            }
        }
    }

    #[inline]
    fn push_task( event_loop: &EventLoop, arc: Arc< Self > ) {
        if !arc.is_queued.swap( true, Ordering::SeqCst ) {
            event_loop.push_task( arc );
        }
    }
}

impl Wake for Task {
    #[inline]
    fn wake( arc: &Arc< Self > ) {
        // TODO maybe store the executor inside the Task ?
        Task::push_task( &EVENT_LOOP, arc.clone() );
    }
}


// A proxy for the JavaScript event loop.
#[derive(Debug)]
struct EventLoop {
    // This avoids unnecessary allocations and interop overhead
    // by using a Rust queue of pending tasks.
    queue: Mutex< VecDeque< Arc< Task > > >,
    is_draining: AtomicBool,

    past_sum: AtomicUsize,
    past_length: AtomicUsize,
    shrink_counter: AtomicUsize,

    // TODO is this thread-safe ?
    waker: Reference,
}

// Not currently necessary, but may become relevant in the future
impl Drop for EventLoop {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            @{&self.waker}.drop();
        }
    }
}

impl EventLoop {
    // Waits for next microtask tick
    fn queue_microtask( &self ) {
        js! { @(no_return) @{&self.waker}(); }
    }

    // Pushes a task onto the queue
    fn push_task( &self, task: Arc< Task > ) {
        let mut queue = self.queue.lock().unwrap();

        queue.push_back( task );

        // If the queue was previously empty, then we need to schedule
        // the queue to be drained.
        //
        // The check for `is_draining` is necessary in the situation where
        // the `drain` method pops the last task from the queue, but that
        // task then re-queues another task.
        if queue.len() == 1 && !self.is_draining.load( Ordering::SeqCst ) {
            self.queue_microtask();
        }
    }

    // See if it's worth trying to reclaim some space from the queue
    fn estimate_realloc_capacity( &self ) -> Option< usize > {
        let queue = self.queue.lock().unwrap();

        let cap = queue.capacity();

        self.past_sum.fetch_add( queue.len(), Ordering::SeqCst );
        self.past_length.fetch_add( 1, Ordering::SeqCst );

        let sum = self.past_sum.load( Ordering::SeqCst );
        let len = self.past_length.load( Ordering::SeqCst );
        let average = sum / len;

        // It will resize the queue if the average length is less than a quarter of the
        // capacity.
        //
        // The check for INITIAL_QUEUE_CAPACITY is necessary in the situation
        // where the queue is at its initial capacity, but the length is very low.
        if average < cap / 4 && cap >= INITIAL_QUEUE_CAPACITY * 2 {
            // It only resizes if the above condition is met for QUEUE_SHRINK_DELAY iterations.
            let shrink_counter = self.shrink_counter.fetch_add( 1, Ordering::SeqCst );

            if shrink_counter >= QUEUE_SHRINK_DELAY {
                self.shrink_counter.store( 0, Ordering::SeqCst );
                return Some( cmp::max( average * 2, INITIAL_QUEUE_CAPACITY ) );
            }

        } else {
            self.shrink_counter.store( 0, Ordering::SeqCst );
        }

        None
    }

    // Poll the queue until it is empty
    fn drain( &self ) {
        if !self.is_draining.swap( true, Ordering::SeqCst ) {
            let maybe_realloc_capacity = self.estimate_realloc_capacity();

            // Poll all the pending tasks
            loop {
                let mut queue = self.queue.lock().unwrap();

                match queue.pop_front() {
                    Some( task ) => {
                        // This is necessary because the polled task might queue more tasks
                        drop( queue );
                        Task::poll( task );
                    },
                    None => {
                        // We decided to reclaim some space
                        if let Some( realloc_capacity ) = maybe_realloc_capacity {
                            *queue = VecDeque::with_capacity( realloc_capacity );
                            // This is necessary because the estimate_realloc_capacity method
                            // relies upon the behavior of the VecDeque's capacity
                            assert!( queue.capacity() < realloc_capacity * 2 );
                        }

                        self.is_draining.store( false, Ordering::SeqCst );

                        break;
                    },
                }
            }
        }
    }
}

lazy_static! {
    #[derive(Debug)]
    static ref EVENT_LOOP: EventLoop = {
        let queue = VecDeque::with_capacity( INITIAL_QUEUE_CAPACITY );
        // This is necessary because the estimate_realloc_capacity method
        // relies upon the behavior of the VecDeque's capacity
        assert!( queue.capacity() < INITIAL_QUEUE_CAPACITY * 2 );

        EventLoop {
            queue: Mutex::new( queue ),
            is_draining: AtomicBool::new( false ),

            past_sum: AtomicUsize::new( 0 ),
            past_length: AtomicUsize::new( 0 ),
            shrink_counter: AtomicUsize::new( 0 ),

            waker: js!(
                var callback = @{|| EVENT_LOOP.drain()};

                var dropped = false;

                function wrapper() {
                    if ( !dropped ) {
                        callback();
                    }
                }

                var nextTick;

                // Modern browsers can use `MutationObserver` which allows
                // us to schedule a micro-task without allocating a promise.
                // https://dom.spec.whatwg.org/#notify-mutation-observers
                if ( typeof MutationObserver === "function" ) {
                    var node = document.createTextNode( "0" );
                    var state = false;

                    new MutationObserver( wrapper ).observe( node, { characterData: true } );

                    nextTick = function () {
                        state = !state;
                        node.data = ( state ? "1" : "0" );
                    };

                // Node.js and other environments
                } else {
                    var promise = Promise.resolve( null );

                    nextTick = function () {
                        promise.then( wrapper );
                    };
                }

                nextTick.drop = function () {
                    dropped = true;
                    callback.drop();
                };

                return nextTick;
            ).try_into().unwrap(),
        }
    };
}

impl<'a> Executor for &'a EVENT_LOOP {
    #[inline]
    fn spawn( &mut self, f: BoxedFuture ) -> Result< (), SpawnError > {
        Task::push_task( &self, Task::new( f ) );
        Ok( () )
    }
}

#[inline]
pub fn spawn< F >( future: F ) where F: Future< Item = (), Error = Never > + 'static + Send {
    (&EVENT_LOOP).spawn( Box::new( future ) ).unwrap();
}
