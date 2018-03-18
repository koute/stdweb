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
    // TODO maybe this should use Weak instead ?
    inner: Rc< Inner >,
}

impl SpawnedTask {
    fn new< F >( future: F, inner: Rc< Inner > ) -> Rc< Self >
        where F: Future< Item = (), Error = () > + 'static {
        Rc::new( Self {
            is_queued: Cell::new( false ),
            spawn: RefCell::new( Some( executor::spawn(
                Box::new( future ) as BoxedFuture
            ) ) ),
            inner,
        } )
    }

    fn poll( &self ) {
        let mut spawn = self.spawn.borrow_mut();

        // Take the future so that if we panic it gets dropped
        if let Some( mut spawn_future ) = spawn.take() {
            // Clear `is_queued` flag so that it will re-queue if poll calls task.notify()
            self.is_queued.set( false );

            if spawn_future.poll_future_notify( &&Notifier, self as *const _ as usize ) == Ok( Async::NotReady ) {
                // Future was not ready, so put it back
                *spawn = Some( spawn_future );
            }
        }
    }

    fn notify( task: Rc< SpawnedTask > ) {
        let inner = &task.inner;

        // If not already queued
        if !task.is_queued.replace( true ) {
            // TODO figure out a way to avoid the clone ?
            inner.queue.queue.borrow_mut().push_back( task.clone() );
        }

        // If not already running
        if !inner.queue.is_running.replace( true ) {
            js! { @(no_return)
                @{&inner.microtask}.next_tick();
            }
        }
    }
}

struct Notifier;

struct Queue {
    is_running: Cell< bool >,
    // TODO maybe SpawnedTask needs to use Arc rather than Rc ?
    queue: RefCell< VecDeque< Rc< SpawnedTask > > >,
}

struct Inner {
    queue: Rc< Queue >,
    microtask: Reference,
}

impl Drop for Inner {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            @{&self.microtask}.callback.drop();
        }
    }
}

struct PromiseExecutor( Rc< Inner > );

// TODO this should be generalized into a MicroTask API
thread_local! {
    static EXECUTOR: PromiseExecutor = {
        let queue = Rc::new( Queue {
            is_running: Cell::new( false ),
            queue: RefCell::new( VecDeque::with_capacity( INITIAL_QUEUE_CAPACITY ) ),
        } );

        let inner = {
            let clone = queue.clone();

            // TODO is Null the fastest type for conversion from JS ?
            let callback = move || {
                loop {
                    let task = clone.queue.borrow_mut().pop_front();

                    if let Some( task ) = task {
                        task.poll();

                    } else {
                        break;
                    }
                }

                // This frees up the memory for the VecDeque
                *clone.queue.borrow_mut() = VecDeque::with_capacity( INITIAL_QUEUE_CAPACITY );

                clone.is_running.set( false );
            };

            Inner {
                queue: queue,
                // This causes the callback to be pushed onto the microtask queue
                microtask: js!(
                    var callback = @{callback};

                    // Modern browsers
                    // https://dom.spec.whatwg.org/#notify-mutation-observers
                    if ( typeof MutationObserver === "function" ) {
                        var node = document.createTextNode( "0" );
                        var state = false;

                        new MutationObserver( function ( changes, observer ) {
                            callback();
                        } ).observe( node, { characterData: true } );

                        return {
                            callback: callback,
                            next_tick: function () {
                                state = !state;
                                node.data = ( state ? "1" : "0" );
                            }
                        };

                    // Node.js and other environments
                    } else {
                        var promise = Promise.resolve( null );

                        // TODO what if the callback has been dropped ?
                        function next_tick( value ) {
                            callback();
                        }

                        return {
                            callback: callback,
                            next_tick: function () {
                                promise.then( next_tick );
                            }
                        };
                    }
                ).try_into().unwrap(),
            }
        };

        PromiseExecutor( Rc::new( inner ) )
    };
}

impl< F > Executor< F > for PromiseExecutor where
    F: Future< Item = (), Error = () > + 'static {
    fn execute( &self, future: F ) -> StdResult< (), ExecuteError< F > > {
        SpawnedTask::notify( SpawnedTask::new( future, self.0.clone() ) );
        Ok( () )
    }
}

impl Notify for Notifier {
    fn notify( &self, spawned_id: usize ) {
        SpawnedTask::notify( unsafe { clone_raw( spawned_id as *const _ ) } );
    }

    // TODO does this cause memory unsafety ?
    fn clone_id( &self, id: usize ) -> usize {
        unsafe { Rc::into_raw( clone_raw( id as *const SpawnedTask ) ) as usize }
    }

    // TODO does this cause memory unsafety ?
    fn drop_id( &self, id: usize ) {
        unsafe { Rc::from_raw( id as *const SpawnedTask ) };
    }
}


#[inline]
pub fn spawn< F >( future: F ) where
    F: Future< Item = (), Error = () > + 'static {
    EXECUTOR.with( |executor| {
        executor.execute( future ).unwrap();
    } );
}
