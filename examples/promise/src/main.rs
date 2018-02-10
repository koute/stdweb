#[macro_use]
extern crate stdweb;
extern crate futures;

use futures::Future;
use stdweb::unstable::{TryInto};
use stdweb::web::error::Error;
use stdweb::{Null, Promise, PromiseFuture};
use std::rc::Rc;
use std::cell::RefCell;
use futures::{Poll, Async};
use futures::task::{current, Task};


fn log( a: &str ) {
    js! { @(no_return)
        console.log( @{a} );
    }
}


fn test_refcell() {
    struct TaskA {
        shared_state: Rc<RefCell<u32>>,
        task_b: Task,
    }

    impl Future for TaskA {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            js! { console.log("Poll TaskA"); }

            let foo = self.shared_state.borrow_mut();

            js! { console.log(@{format!("TaskA 1: {:#?}", foo)}); }

            self.task_b.notify();

            js! { console.log(@{format!("TaskA 2: {:#?}", foo)}); }

            Ok(Async::NotReady)
        }
    }

    struct TaskB {
        shared_state: Rc<RefCell<u32>>,
        initialized: bool,
    }

    impl Future for TaskB {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            js! { console.log("Poll TaskB"); }

            if !self.initialized {
                self.initialized = true;

                let task_b = current();

                let foo = self.shared_state.borrow();

                js! { console.log(@{format!("TaskB 1: {:#?}", foo)}); }

                PromiseFuture::spawn(TaskA {
                    shared_state: self.shared_state.clone(),
                    task_b: task_b,
                });
            }

            let foo = self.shared_state.borrow();

            js! { console.log(@{format!("TaskB 1: {:#?}", foo)}); }

            Ok(Async::NotReady)
        }
    }

    PromiseFuture::spawn(TaskB {
        shared_state: Rc::new(RefCell::new(0)),
        initialized: false
    });
}


fn test_panic() {
    let promise: Promise = js!( return Promise.resolve(null); ).try_into().unwrap();

    promise.done( |result: Result< Null, Error >| {
        log( &format!( "Promise result: {:#?}", result ) );
        panic!( "Testing panic!" );
    } );
}


fn test_notify() {
    struct MyFuture {
        polls: u32,
        count: u32,
        done: bool,
        receiver: futures::unsync::oneshot::Receiver< () >,
    }

    impl MyFuture {
        fn new( count: u32 ) -> Self {
            let ( sender, receiver ) = futures::unsync::oneshot::channel();

            let callback = || {
                log( "setTimeout done" );

                log( &format!("Sending {:#?}", sender.send( () ) ) );
            };

            log( "setTimeout started" );

            js! { @(no_return)
                setTimeout( function () {
                    @{stdweb::Once( callback )}();
                }, 1000 );
            }

            Self {
                polls: 0,
                count: count,
                done: false,
                receiver,
            }
        }
    }

    impl Future for MyFuture {
        type Item = u32;
        type Error = ();

        fn poll( &mut self ) -> futures::Poll< Self::Item, Self::Error > {
            self.polls += 1;

            if !self.done {
                match self.receiver.poll() {
                    Ok( futures::Async::Ready( () ) ) => self.done = true,

                    Ok( futures::Async::NotReady ) => {},

                    Err( _ ) => self.done = true,
                }
            }

            if self.done {
                if self.count == 0 {
                    Ok( futures::Async::Ready( self.polls ) )

                } else {
                    self.count -= 1;

                    let task = futures::task::current();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();
                    task.notify();

                    Ok( futures::Async::NotReady )
                }

            } else {
                Ok( futures::Async::NotReady )
            }
        }
    }

    PromiseFuture::spawn(
        MyFuture::new( 5 ).map( |x| {
            log( &format!( "MyFuture count: {}", x ) );
            assert_eq!( x, 7 );
        } )
    );
}


fn test_timeout() {
    fn sleep( ms: u32 ) -> PromiseFuture< Null > {
        js!( return new Promise( function ( success, failure ) {
            setTimeout( function () {
                success( null );
            }, @{ms} );
        } ); ).try_into().unwrap()
    }

    PromiseFuture::spawn(
        sleep( 2000 ).inspect( |_| log( "Timeout 1 done!") ).join(
        sleep( 2000 ).inspect( |_| log( "Timeout 2 done!" ) ) )
            .and_then( |_|
                sleep( 1000 ).inspect( |_| log( "Timeout 3 done!") ) )
            .and_then( |_|
                futures::future::err( Error::new( "Testing error!" ) ) )
            .map_err( |e| e.print() )
    );
}


fn main() {
    stdweb::initialize();

    test_refcell();
    test_panic();
    test_notify();
    test_timeout();

    stdweb::event_loop();
}

