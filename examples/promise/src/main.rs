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


fn test_from_thenable() {
    let a = Promise::from_thenable( &js!( return Promise.resolve(5); ).try_into().unwrap() );

    a.unwrap().done( |result: Result< u32, u32 > | {
        assert_eq!( result, Ok( 5 ) );
        console!( log, format!( "Thenable 1: {:#?}", result ) );
    } );


    let a = Promise::from_thenable( &js!( return { then: function (yes, no) { yes( 1 ); } }; ).try_into().unwrap() );

    a.unwrap().done( |result: Result< u32, u32 > | {
        assert_eq!( result, Ok( 1 ) );
        console!( log, format!( "Thenable 2: {:#?}", result ) );
    } );


    let a = Promise::from_thenable( &js!( return {}; ).try_into().unwrap() );

    assert!( a.is_none() );
}


fn test_error_conversion() {
    let a: PromiseFuture< Null, String > = js!( return Promise.reject( "hi!" ); ).try_into().unwrap();

    PromiseFuture::spawn(
        a.map( |_| () ).map_err( |x| {
            console!( log, "String error:", x );
        } )
    );

    let _a: PromiseFuture< Null > = js!( return Promise.resolve( null ); ).try_into().unwrap();
    console!( log, "Null works" );

    let _a: PromiseFuture< Null > = js!( return Promise.reject( new Error( "hi!" ) ); ).try_into().unwrap();
    console!( log, "Error works" );

    //let _a: PromiseFuture< Null, SyntaxError > = js!( return Promise.reject( new Error( "hi!" ) ); ).try_into().unwrap();
    //console!( log, "Error conversion fails" );
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
            console!( log, "Poll TaskA" );

            let foo = self.shared_state.borrow_mut();

            console!( log, format!("TaskA 1: {:#?}", foo) );

            self.task_b.notify();

            console!( log, format!("TaskA 2: {:#?}", foo) );

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
            console!( log, "Poll TaskB" );

            if !self.initialized {
                self.initialized = true;

                let task_b = current();

                let foo = self.shared_state.borrow();

                console!( log, format!("TaskB 1: {:#?}", foo) );

                PromiseFuture::spawn(TaskA {
                    shared_state: self.shared_state.clone(),
                    task_b: task_b,
                });
            }

            let foo = self.shared_state.borrow();

            console!( log, format!("TaskB 1: {:#?}", foo) );

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
        console!( log, format!( "Promise result: {:#?}", result ) );
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
                console!( log, "setTimeout done" );

                console!( log, format!("Sending {:#?}", sender.send( () ) ) );
            };

            console!( log, "setTimeout started" );

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
            console!( log, format!( "MyFuture count: {}", x ) );
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
        sleep( 2000 ).inspect( |_| console!( log, "Timeout 1 done!" ) ).join(
        sleep( 2000 ).inspect( |_| console!( log, "Timeout 2 done!" ) ) )
            .and_then( |_|
                sleep( 1000 ).inspect( |_| console!( log, "Timeout 3 done!" ) ) )
            .and_then( |_|
                futures::future::err( Error::new( "Testing error!" ) ) )
            .map_err( |e| console!( error, e ) )
    );
}


fn main() {
    stdweb::initialize();

    test_from_thenable();
    test_refcell();
    test_panic();
    test_notify();
    test_timeout();
    test_error_conversion();

    stdweb::event_loop();
}
