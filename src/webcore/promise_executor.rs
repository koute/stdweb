use futures::future::{Future, ExecuteError, Executor};
use futures::executor::{self, Notify, Spawn};
use futures::Async;
use std::result::Result as StdResult;
use std::cell::{Cell, RefCell};


#[derive(Clone, Copy)]
enum TaskState {
    Idle,
    Running,
    Queued,
}

struct SpawnedTask {
    state: Cell< TaskState >,
    // TODO use Rc<SpawnedTask> instead ?
    ref_count: Cell< usize >,
    spawn: RefCell< Spawn< Box< Future< Item = (), Error = () > + 'static > > >,
}

impl SpawnedTask {
    fn new< F >( future: F ) -> Self
        where F: Future< Item = (), Error = () > + 'static {
        Self {
            state: Cell::new( TaskState::Idle ),
            ref_count: Cell::new( 1 ),
            spawn: RefCell::new( executor::spawn(
                Box::new( future ) as Box< Future< Item = (), Error = () > + 'static >
            ) ),
        }
    }

    // These are the situations that this algorithm needs to worry about:
    //
    //   execute -> poll
    //   execute -> poll + poll
    //   execute -> poll + notify -> repoll
    //   execute -> poll + notify + notify -> repoll
    //   execute -> poll + notify -> repoll -> async notify -> repoll
    //   execute -> poll -> async notify -> repoll
    //   execute -> poll -> async notify -> repoll + notify -> repoll
    //   execute -> poll -> async notify -> repoll -> async notify -> repoll
    //
    unsafe fn execute_spawn( spawned_ptr: *const SpawnedTask ) {
        let spawned = &*spawned_ptr;

        loop {
            // This is necessary because the Future might call `task.notify()` inside of `poll`.
            // If that happens, we need to re-run `poll` again. So we use `state` to indicate
            // whether we need to re-run `poll` or not.
            spawned.state.set( TaskState::Running );

            // Here we try to call `poll` on the future. This may not be possible,
            // as the future may already be executed somewhere higher up in the stack.
            // We know there is at least one execution scheduled, so it's styled more
            // like a do-while loop.
            //
            // The usage of the lock needs to be contained in a scope that is
            // separate from the decrement_ref_count, as that can deallocate the
            // whole spawned task, causing a segfault when the RefCell is trying
            // to release its borrow. That's why the poll_future_notify call is
            // contained inside the map.
            let result = spawned
                .spawn
                .try_borrow_mut()
                .map( |mut s| s.poll_future_notify( &CORE, spawned_ptr as usize ) );

            // TODO test this
            if let Ok( result ) = result {
                if let Ok( Async::NotReady ) = result {
                    // The `poll` method called `task.notify()` so we have to re-run `poll` again.
                    if let TaskState::Queued = spawned.state.get() {
                        continue;

                    } else {
                        // The Future isn't ready yet, but it will asynchronously
                        // call `task.notify()` when it is ready.
                        //
                        // When that happens, the `notify` function will call
                        // `execute_spawn` again.
                        spawned.state.set( TaskState::Idle );
                    }

                } else {
                    // This ensures that the Future will never be polled again.
                    spawned.state.set( TaskState::Running );

                    // The whole object might be deallocated at this point, so
                    // it would be very dangerous to touch anything else.
                    SpawnedTask::decrement_ref_count( spawned_ptr as usize );
                }
            }

            return;
        }
    }

    unsafe fn notify( spawned_ptr: *const SpawnedTask ) {
        let spawned = &*spawned_ptr;

        // This causes it to re-run `poll` again, even if `task.notify()` was called inside of `poll`.
        //
        // IMPORTANT: If the Future calls `notify` multiple times within `poll`, it will only re-run
        //            `poll` once!
        let state = spawned.state.replace( TaskState::Queued );

        // This only happens when `task.notify()` is called asynchronously.
        if let TaskState::Idle = state {
            SpawnedTask::execute_spawn( spawned_ptr );
        }
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

            // This causes the SpawnedTask to be dropped at the end of the scope.
            let spawned = Box::from_raw( spawned_ptr );

            // This ensures that the Future will never be polled again.
            spawned.state.set( TaskState::Running );
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
        unsafe {
            SpawnedTask::notify( spawned_id as *const SpawnedTask );
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
    CORE.execute( future ).unwrap();
}
