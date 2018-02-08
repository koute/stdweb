use futures::future::{Future, ExecuteError, Executor};
use futures::executor::{self, Notify, Spawn};
use futures::Async;
use std::result::Result as StdResult;
use std::cell::{Cell, RefCell};
use Once;


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

        unsafe fn run( spawned_ptr: *const SpawnedTask ) {
            let spawned = &*spawned_ptr;

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

                if let Ok( Async::NotReady ) = result {
                    // If there are more queued executions, then we execute them on the next event tick.
                    // This is necessary because the Future might be waiting for an event on the event loop.
                    if spawned.resubmission_count.get() != 0 {
                        let callback = move || run( spawned_ptr );

                        // TODO setTimeout isn't available in all JavaScript environments
                        js! { @(no_return)
                            setTimeout( @{Once( callback )}, 0 );
                        }
                    }

                } else {
                    // The whole object might be deallocated at this point, so
                    // it would be very dangerous to touch anything else.
                    SpawnedTask::decrement_ref_count( spawned_ptr as usize );
                }
            }
        }

        run( spawned_ptr );
    }

    unsafe fn increment_ref_count( id: usize ) -> usize {
        let spawned_ptr = id as *const SpawnedTask;
        let spawned = &*spawned_ptr;
        let mut count = spawned.ref_count.get();
        count += 1;
        spawned.ref_count.set( count );
        id
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

            // This causes the SpawnedTask to be dropped
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
        unsafe {
            SpawnedTask::increment_ref_count( id )
        }
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
