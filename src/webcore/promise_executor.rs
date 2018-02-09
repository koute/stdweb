use futures::future::{Future, ExecuteError, Executor};
use futures::executor::{self, Notify, Spawn};
use futures::Async;
use std::result::Result as StdResult;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use webcore::once::Once;

// This functionality should really be in libstd, because the implementation
// looks stupid.
unsafe fn clone_raw<T>(ptr: *const T) -> Rc<T> {
    let result = Rc::from_raw(ptr);
    ::std::mem::forget(result.clone());
    result
}

// Typing this out is tedious
type BoxedFuture = Box< Future< Item = (), Error = () > + 'static >;

struct SpawnedTask {
    is_queued: Cell< bool >,
    spawn: RefCell< Option< Spawn< BoxedFuture > >  >,
}

impl SpawnedTask {
    fn new< F >( future: F ) -> Rc<Self>
        where F: Future< Item = (), Error = () > + 'static {
        Rc::new(Self {
            is_queued: Cell::new( false ),
            spawn: RefCell::new( Some( executor::spawn(
                Box::new( future ) as BoxedFuture
            ) ) ),
        })
    }

    fn poll(&self) {
        let mut spawn = self.spawn.borrow_mut();

        // Take the future so that if we panic it gets dropped
        if let Some(mut spawn_future) = spawn.take() {
            // Clear `is_queued` flag
            self.is_queued.set(false);

            if spawn_future.poll_future_notify( &&Core, self as *const _ as usize ) == Ok(Async::NotReady) {
                // Future was not ready, so put it back
                *spawn = Some(spawn_future);
            }
        }
    }

    fn notify( spawned: Rc<SpawnedTask> ) {
        // If not already queued
        if !spawned.is_queued.replace(true) {
            // Poll on next micro-task
            js! { @(no_return)
                Promise.resolve().then(function() {
                    @{ Once(move || spawned.poll()) }();
                });
            }
        }
    }
}

struct Core;

impl< F > Executor< F > for Core where
    F: Future< Item = (), Error = () > + 'static {
    fn execute( &self, future: F ) -> StdResult< (), ExecuteError< F > > {
        SpawnedTask::notify( SpawnedTask::new( future ) );
        Ok( () )
    }
}

impl Notify for Core {
    fn notify( &self, spawned_id: usize ) {
        SpawnedTask::notify(unsafe { clone_raw(spawned_id as *const _) })
    }

    fn clone_id( &self, id: usize ) -> usize {
        unsafe { Rc::into_raw(clone_raw(id as *const SpawnedTask)) as usize }
    }

    fn drop_id( &self, id: usize ) {
        unsafe { Rc::from_raw(id as *const SpawnedTask) };
    }
}


#[inline]
pub fn spawn< F >( future: F ) where
    F: Future< Item = (), Error = () > + 'static {
    Core.execute( future ).unwrap();
}
