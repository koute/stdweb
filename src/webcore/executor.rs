// This file implements a futures-compatible executor which schedules futures
// onto the JavaScript event loop.
//
// TODO: Verify that this works correctly for multiple threads.
// TODO: verify that this works correctly with pinned futures
// TODO: use FuturesUnordered (similar to LocalPool)

use futures_core::future::{FutureObj, LocalFutureObj};
use futures_executor::enter;
use futures_core::task::{Spawn, SpawnError};
use futures_util::task::{self, ArcWake};
use std::future::Future;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::collections::VecDeque;
use std::cmp;
use webcore::try_from::TryInto;
use webcore::value::Reference;


// TODO: Determine optimal values for these constants
// Initial capacity of the event queue
const INITIAL_QUEUE_CAPACITY: usize = 10;

// Iterations to wait before allowing the queue to shrink
const QUEUE_SHRINK_DELAY: usize = 10;


pub(crate) type BoxedFuture = LocalFutureObj< 'static, () >;

#[derive(Debug)]
struct TaskInner {
    future: BoxedFuture,
    executor: EventLoopExecutor,
}


#[derive(Debug)]
struct Task {
    is_queued: Cell< bool >,
    inner: RefCell< TaskInner >,
}

// TODO fix these
// It is safe because JavaScript is (currently) always single-threaded
unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    fn new( executor: EventLoopExecutor, future: BoxedFuture ) -> Arc< Self > {
        Arc::new( Self {
            is_queued: Cell::new( true ),
            inner: RefCell::new( TaskInner {
                future,
                executor,
            } ),
        } )
    }

    fn poll( arc: Arc< Self > ) {
        let mut lock = arc.inner.borrow_mut();

        // This is needed in order to borrow disjoint struct fields
        let lock = &mut *lock;

        // Clear `is_queued` flag so that it will re-queue if poll calls waker.wake()
        arc.is_queued.set( false );

        let poll = {
            // TODO is there some way of saving these so they don't need to be recreated all the time ?
            let waker = task::waker( arc.clone() );
            let cx = &mut Context::from_waker( &waker );

            // TODO what if poll panics ?
            // TODO is this Pin correct ?
            Pin::new( &mut lock.future ).poll( cx )
        };

        if let Poll::Pending = poll {
            // It was woken up during the poll, so we requeue it
            if arc.is_queued.get() {
                lock.executor.0.push_task( arc.clone() );
            }
        }
    }

    #[inline]
    fn push_task( arc: &Arc< Self > ) {
        if !arc.is_queued.replace( true ) {
            if let Ok( lock ) = arc.inner.try_borrow() {
                lock.executor.0.push_task( arc.clone() );
            }
        }
    }
}

impl ArcWake for Task {
    #[inline]
    fn wake_by_ref( arc_self: &Arc< Self > ) {
        Task::push_task( arc_self );
    }
}


#[derive(Debug)]
struct EventLoopInner {
    // This avoids unnecessary allocations and interop overhead
    // by using a Rust queue of pending tasks.
    queue: VecDeque< Arc< Task > >,
    // TODO handle overflow
    past_sum: usize,
    past_length: usize,
    shrink_counter: usize,
}

#[derive(Debug)]
struct EventLoopQueue {
    inner: RefCell< EventLoopInner >,
    is_draining: Cell< bool >,
}

impl EventLoopQueue {
    // See if it's worth trying to reclaim some space from the queue
    fn estimate_realloc_capacity( &self ) -> Option< ( usize, usize ) > {
        let mut inner = self.inner.borrow_mut();

        let cap = inner.queue.capacity();

        inner.past_sum += inner.queue.len();
        inner.past_length += 1;

        let average = inner.past_sum / inner.past_length;

        // It will resize the queue if the average length is less than a quarter of the
        // capacity.
        //
        // The check for INITIAL_QUEUE_CAPACITY is necessary in the situation
        // where the queue is at its initial capacity, but the length is very low.
        if average < cap / 4 && cap >= INITIAL_QUEUE_CAPACITY * 2 {
            // It only resizes if the above condition is met for QUEUE_SHRINK_DELAY iterations.
            inner.shrink_counter += 1;

            if inner.shrink_counter >= QUEUE_SHRINK_DELAY {
                inner.shrink_counter = 0;
                return Some( ( cap, cmp::max( average * 2, INITIAL_QUEUE_CAPACITY ) ) );
            }

        } else {
            inner.shrink_counter = 0;
        }

        None
    }

    // Poll the queue until it is empty
    fn drain( &self ) {
        let _enter = enter().expect( "EventLoopExecutor is already draining" );

        if !self.is_draining.replace( true ) {
            let maybe_realloc_capacity = self.estimate_realloc_capacity();

            // Poll all the pending tasks
            loop {
                let mut inner = self.inner.borrow_mut();

                match inner.queue.pop_front() {
                    Some( task ) => {
                        // This is necessary because the polled task might queue more tasks
                        drop( inner );
                        Task::poll( task );
                    },
                    None => {
                        // We decided to reclaim some space
                        if let Some( ( old_capacity, realloc_capacity ) ) = maybe_realloc_capacity {
                            inner.queue = VecDeque::with_capacity( realloc_capacity );

                            let new_capacity = inner.queue.capacity();

                            // This makes sure that we are actually shrinking the capacity
                            assert!( new_capacity < old_capacity );

                            // This is necessary because the estimate_realloc_capacity method
                            // relies upon the behavior of the VecDeque's capacity
                            assert!( new_capacity < realloc_capacity * 2 );
                        }

                        self.is_draining.set( false );

                        break;
                    },
                }
            }
        }
    }
}


// A proxy for the JavaScript event loop.
#[derive(Debug)]
struct EventLoop {
    queue: Rc< EventLoopQueue >,
    // TODO is this thread-safe ?
    waker: Reference,
}

impl EventLoop {
    // Waits for next microtask tick
    fn queue_microtask( &self ) {
        js! { @(no_return) @{&self.waker}(); }
    }

    // Pushes a task onto the queue
    fn push_task( &self, task: Arc< Task > ) {
        let mut inner = self.queue.inner.borrow_mut();

        inner.queue.push_back( task );

        // If the queue was previously empty, then we need to schedule
        // the queue to be drained.
        //
        // The check for `is_draining` is necessary in the situation where
        // the `drain` method pops the last task from the queue, but that
        // task then re-queues another task.
        if inner.queue.len() == 1 && !self.queue.is_draining.get() {
            self.queue_microtask();
        }
    }
}

// Not currently necessary, but may become relevant in the future
// TODO what about when the thread is killed, is this guaranteed to be called ?
impl Drop for EventLoop {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            @{&self.waker}.drop();
        }
    }
}


#[derive(Debug, Clone)]
struct EventLoopExecutor( Rc< EventLoop > );

impl EventLoopExecutor {
    fn new() -> Self {
        let queue = VecDeque::with_capacity( INITIAL_QUEUE_CAPACITY );
        // This is necessary because the estimate_realloc_capacity method
        // relies upon the behavior of the VecDeque's capacity
        assert!( queue.capacity() < INITIAL_QUEUE_CAPACITY * 2 );

        let queue = Rc::new( EventLoopQueue {
            inner: RefCell::new( EventLoopInner {
                queue: queue,
                past_sum: 0,
                past_length: 0,
                shrink_counter: 0,
            } ),

            is_draining: Cell::new( false ),
        } );

        let waker = {
            let queue = queue.clone();

            js!(
                var callback = @{move || queue.drain()};

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
            ).try_into().unwrap()
        };

        EventLoopExecutor( Rc::new( EventLoop { queue, waker } ) )
    }

    #[inline]
    fn spawn_local( &self, future: BoxedFuture ) {
        self.0.push_task( Task::new( self.clone(), future ) );
    }
}

impl Spawn for EventLoopExecutor {
    #[inline]
    fn spawn_obj( &mut self,  future: FutureObj< 'static, () > ) -> Result< (), SpawnError > {
        self.spawn_local( future.into() );
        Ok( () )
    }
}


pub(crate) fn spawn_local( future: BoxedFuture ) {
    thread_local! {
        static EVENT_LOOP: EventLoopExecutor = EventLoopExecutor::new();
    }

    EVENT_LOOP.with( |event_loop| event_loop.spawn_local( future ) )
}
