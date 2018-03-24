use futures::future::{Future, ExecuteError, Executor};
use futures::executor::{self, Notify, Spawn};
use futures::Async;
use std::collections::VecDeque;
use std::result::Result as StdResult;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use webcore::try_from::TryInto;
use webcore::value::Reference;


const INITIAL_QUEUE_CAPACITY: usize = 10;


// This functionality should really be in libstd, because the implementation
// looks stupid.
unsafe fn clone_raw< T >( ptr: *const T ) -> Rc< T > {
    let result = Rc::from_raw( ptr );
    ::std::mem::forget( result.clone() );
    result
}

// Typing this out is tedious
type BoxedFuture = Box< Future< Item = (), Error = () > + 'static >;

struct SpawnedTask {
    is_queued: Cell< bool >,
    spawn: RefCell< Option< Spawn< BoxedFuture > > >,
}

impl SpawnedTask {
    fn new< F >( future: F ) -> Rc< Self >
        where F: Future< Item = (), Error = () > + 'static {
        Rc::new( Self {
            is_queued: Cell::new( false ),
            spawn: RefCell::new( Some( executor::spawn(
                Box::new( future ) as BoxedFuture
            ) ) )
        } )
    }

    fn poll( &self ) {
        let mut spawn = self.spawn.borrow_mut();

        // Take the future so that if we panic it gets dropped
        if let Some( mut spawn_future ) = spawn.take() {
            // Clear `is_queued` flag so that it will re-queue if poll calls task.notify()
            self.is_queued.set( false );

            if spawn_future.poll_future_notify( &&EventLoop, self as *const _ as usize ) == Ok( Async::NotReady ) {
                // Future was not ready, so put it back
                *spawn = Some( spawn_future );
            }
        }
    }

    fn notify( task: Rc< SpawnedTask > ) {
        // If not already queued
        if !task.is_queued.replace( true ) {
            EventLoop.push_task(task);
        }
    }
}

// A proxy for the JavaScript event loop.
struct EventLoop;

// There's only one thread, but this lets us tell the compiler that we
// don't need a `Sync` bound, and also gives us lazy initialization.
thread_local! {
    static EVENT_LOOP_INNER: EventLoopInner = EventLoopInner::new();
}

impl EventLoop {
    fn drain(&self) {
        EVENT_LOOP_INNER.with(EventLoopInner::drain)
    }
    fn push_task(&self, task: Rc< SpawnedTask >) {
        EVENT_LOOP_INNER.with(|inner| inner.push_task(task))
    }
}

// State relating to the JavaScript event loop. Only one instance ever exists.
struct EventLoopInner {
    // Avoid unnecessary allocation and interop by keeping a local
    // queue of pending tasks.
    microtask_queue: RefCell< VecDeque< Rc< SpawnedTask > > >,
    waker: Reference,
}

// Not strictly necessary, but may become relevant in the future
impl Drop for EventLoopInner {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            @{&self.waker}.drop();
        }
    }
}

impl EventLoopInner {
    // Initializes the event loop. Only called once.
    fn new() -> Self {
        EventLoopInner {
            microtask_queue: RefCell::new(VecDeque::with_capacity(INITIAL_QUEUE_CAPACITY)),
            waker: js!(
                var callback = @{|| EventLoop.drain()};
                var wrapper = function() {
                    if (!callback.dropped) { callback() }
                };

                // Modern browsers can use `MutationObserver` which allows
                // us to schedule a micro-task without allocating a promise.
                // https://dom.spec.whatwg.org/#notify-mutation-observers
                if ( typeof MutationObserver === "function" ) {
                    var node = document.createTextNode( "0" );
                    var state = false;

                    new MutationObserver( wrapper ).observe( node, { characterData: true } );

                    function nextTick() {
                        state = !state;
                        node.data = ( state ? "1" : "0" );
                    }

                // Node.js and other environments
                } else {
                    var promise = Promise.resolve( null );

                    function nextTick() {
                        promise.then( wrapper );
                    }
                }

                nextTick.drop = function() {
                    callback.dropped = true;
                    callback.drop();
                };

                return nextTick;
            ).try_into().unwrap()
        }
    }
    // Pushes a task onto the queue
    fn push_task(&self, task: Rc< SpawnedTask >) {
        let mut queue = self.microtask_queue.borrow_mut();
        queue.push_back(task);

        // If the queue was previously empty, then we need to schedule
        // the queue to be drained.
        if queue.len() == 1 {
            self.wake();
        }
    }
    // Invoke the JavaScript waker function
    fn wake(&self) {
        js! { @(no_return) @{&self.waker}(); }
    }
    // Remove and return a task from the front of the queue
    fn pop_task(&self) -> Option< Rc< SpawnedTask > > {
        self.microtask_queue.borrow_mut().pop_front()
    }
    // Poll the queue until it is empty
    fn drain(&self) {
        while let Some(task) = self.pop_task() {
            task.poll();
        }
    }
}

impl< F > Executor< F > for EventLoop where
    F: Future< Item = (), Error = () > + 'static {
    fn execute( &self, future: F ) -> StdResult< (), ExecuteError< F > > {
        SpawnedTask::notify( SpawnedTask::new( future ) );
        Ok( () )
    }
}

impl Notify for EventLoop {
    fn notify( &self, spawned_id: usize ) {
        SpawnedTask::notify( unsafe { clone_raw( spawned_id as *const _ ) } );
    }

    fn clone_id( &self, id: usize ) -> usize {
        unsafe { Rc::into_raw( clone_raw( id as *const SpawnedTask ) ) as usize }
    }

    fn drop_id( &self, id: usize ) {
        unsafe { Rc::from_raw( id as *const SpawnedTask ) };
    }
}

#[inline]
pub fn spawn< F >( future: F ) where
    F: Future< Item = (), Error = () > + 'static {
    EventLoop.execute( future ).unwrap();
}
