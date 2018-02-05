use futures::future::{Future, ExecuteError, Executor};
use futures::executor::{self, Notify, Spawn};
use futures::Async;
use std::result::Result as StdResult;
use std::cell::{Cell, RefCell};


struct SpawnedTask {
    ref_count: Cell< usize >,
    spawn: RefCell< Spawn< Box< Future<Item = (), Error = () > + 'static > > >,
}

impl SpawnedTask {
    fn new< F >( future: F ) -> Self
        where F: Future< Item = (), Error = () > + 'static {
        Self {
            ref_count: Cell::new( 1 ),
            spawn: RefCell::new( executor::spawn( Box::new( future.fuse() )
                as Box< Future<Item = (), Error = () > + 'static> ) ),
        }
    }

    fn execute_spawn( spawned_ptr: *const SpawnedTask ) {
        let spawned = unsafe { &*spawned_ptr };

        // This is probably suboptimal, as a resubmission of the same Task while it
        // is being executed results in a panic. It is not entirely clear if a Task
        // is allowed to do that, but I would expect that this is valid behavior, as
        // the notification could happen while the Task is still executing, in a
        // truly multi-threaded situation. So we probably have to deal with it here
        // at some point too. This already happened in the IntervalStream, so that
        // should be cleaned up then as well then. The easiest solution is to try to
        // lock it instead and if it fails, increment a counter. The one that
        // initially blocked the RefCell then just reexecutes the Task until the
        // Task is finished or the counter reaches 0.

        if spawned.spawn.borrow_mut().poll_future_notify( &CORE, spawned_ptr as usize ) != Ok( Async::NotReady ) {
            SpawnedTask::decrement_ref_count( spawned_ptr as usize );
        }
    }

    fn decrement_ref_count( id: usize ) {
        let count = {
            let spawned_ptr = id as *const SpawnedTask;
            let spawned = unsafe { &*spawned_ptr };
            let mut count = spawned.ref_count.get();
            count -= 1;
            spawned.ref_count.set( count );
            count
        };

        if count == 0 {
            let spawned_ptr = id as *mut SpawnedTask;
            unsafe { Box::from_raw( spawned_ptr ) };
        }
    }
}


static CORE: &Core = &Core;

struct Core;

impl< F > Executor< F > for Core where
    F: Future< Item = (), Error = () > + 'static {
    fn execute( &self, future: F ) -> StdResult< (), ExecuteError< F > > {
        let spawned_ptr = Box::into_raw( Box::new( SpawnedTask::new( future ) ) );

        SpawnedTask::execute_spawn( spawned_ptr );

        Ok( () )
    }
}

impl Notify for Core {
    fn notify( &self, spawned_id: usize ) {
        let spawned_ptr = spawned_id as *const SpawnedTask;

        SpawnedTask::execute_spawn( spawned_ptr );
    }

    fn clone_id( &self, id: usize ) -> usize {
        let spawned_ptr = id as *const SpawnedTask;
        let spawned = unsafe { &*spawned_ptr };
        let mut count = spawned.ref_count.get();
        count += 1;
        spawned.ref_count.set( count );
        id
    }

    fn drop_id( &self, id: usize ) {
        SpawnedTask::decrement_ref_count( id );
    }
}


#[inline]
pub fn spawn< F >( future: F ) where
    F: Future< Item = (), Error = () > + 'static {
    CORE.execute( future ).ok();
}