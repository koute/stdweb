use futures::future::{Future, ExecuteError, Executor};
use futures::executor::{self, Notify, Spawn};
use futures::Async;
use std::result::Result as StdResult;
use std::cell::{Cell, RefCell};


struct SpawnedTask {
    ref_count: Cell< usize >,
    resubmission_count: Cell< usize >,
    spawn: RefCell< Spawn< Box< Future<Item = (), Error = () > + 'static > > >,
}

impl SpawnedTask {
    fn new< F >( future: F ) -> Self
        where F: Future< Item = (), Error = () > + 'static {
        Self {
            ref_count: Cell::new( 1 ),
            resubmission_count: Cell::new( 0 ),
            spawn: RefCell::new( executor::spawn( Box::new( future.fuse() )
                as Box< Future<Item = (), Error = () > + 'static> ) ),
        }
    }

    unsafe fn execute_spawn( spawned_ptr: *const SpawnedTask ) {
        let spawned = &*spawned_ptr;

        // Queue up the task for execution.
        spawned
            .resubmission_count
            .set( spawned.resubmission_count.get() + 1 );

        loop {
            // Here we try to take an execution token from the queue and execute
            // the future. This may not be possible, as the future may already
            // be executed somewhere higher up in the stack. We know there is at
            // least one execution scheduled, so it's styled more like a
            // do-while loop.

            // The usage of the lock needs to be contained in a scope that is
            // separate from the decrement_ref_count, as that can deallocate the
            // whole spawned task, causing a segfault when the RefCell is trying
            // to release its borrow. That's why the poll_future_notify call is
            // contained inside the map.

            let result = spawned
                .spawn
                .try_borrow_mut()
                .map( |mut s| s.poll_future_notify( &CORE, spawned_ptr as usize ) );

            if let Ok( result ) = result {
                // We were able to successfully execute the future, allowing us
                // to dequeue one resubmission token.
                spawned
                    .resubmission_count
                    .set( spawned.resubmission_count.get() - 1 );

                if result != Ok( Async::NotReady ) {
                    SpawnedTask::decrement_ref_count( spawned_ptr as usize );

                    // Return out early. The whole object might be deallocated
                    // at this point, so it would be very dangerous to touch
                    // anything else.
                    return;
                }

                if spawned.resubmission_count.get() == 0 {
                    // Looks like there is no additional executions queued up.
                    // We can end the execution loop here.
                    return;
                }
            } else {
                // We failed to execute the Task as it is already being executed
                // higher up in the stack. We don't consume our execution token,
                // and just leave it for the Task execution higher up to
                // consume. We can't do anything anymore, so we yield execution
                // back to the caller.
                return;
            }
        }
    }

    unsafe fn decrement_ref_count( id: usize ) {
        let count = {
            let spawned_ptr = id as *const SpawnedTask;
            let spawned = &*spawned_ptr;
            let mut count = spawned.ref_count.get();
            count -= 1;
            spawned.ref_count.set( count );
            count
        };

        if count == 0 {
            let spawned_ptr = id as *mut SpawnedTask;
            Box::from_raw( spawned_ptr );
        }
    }
}


static CORE: &Core = &Core;

struct Core;

impl< F > Executor< F > for Core where
    F: Future< Item = (), Error = () > + 'static {
    fn execute( &self, future: F ) -> StdResult< (), ExecuteError< F > > {
        let spawned_ptr = Box::into_raw( Box::new( SpawnedTask::new( future ) ) );

        unsafe {
            SpawnedTask::execute_spawn( spawned_ptr );
        }

        Ok( () )
    }
}

impl Notify for Core {
    fn notify( &self, spawned_id: usize ) {
        let spawned_ptr = spawned_id as *const SpawnedTask;

        unsafe {
            SpawnedTask::execute_spawn( spawned_ptr );
        }
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
        unsafe {
            SpawnedTask::decrement_ref_count( id );
        }
    }
}


#[inline]
pub fn spawn< F >( future: F ) where
    F: Future< Item = (), Error = () > + 'static {
    CORE.execute( future ).ok();
}
