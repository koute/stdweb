use std;
use std::error::Error;
use std::marker::PhantomData;
use webcore::once::Once;
use webcore::value::{Value, Reference, ConversionError};
use webcore::try_from::{TryInto, TryFrom};
use web::error::Error as JSError;
use futures::{future, Future, Poll};
use futures::unsync::oneshot::channel;


pub struct Promise( Reference );

reference_boilerplate! {
    Promise,
    instanceof Promise
}

impl Promise {
    pub fn promisify( input: Value ) -> Promise {
        js!( return Promise.resolve( @{input} ); ).try_into().unwrap()
    }

    pub fn done< A, B >( &self, callback: B )
        where A: TryFrom< Value >,
              A::Error: Error,
              B: FnOnce( Result< A, JSError > ) + 'static {

        let callback = |value: Value, success: bool| {
            let value: Result< A, JSError > = if success {
                let value: Result< A, A::Error > = value.try_into();
                value.map_err( |e| JSError::new( e.description() ) )
            } else {
                let value: Result< JSError, ConversionError > = value.try_into();
                value.map_err( |e| JSError::new( e.description() ) ).and_then( Err )
            };

            callback( value );
        };

        js! { @(no_return)
            var callback = @{Once( callback )};

            // TODO don't swallow any errors thrown inside callback
            @{self}.then( function (value) {
                callback( value, true );
            }, function (value) {
                callback( value, false );
            } );
        }
    }

    // We can't use the IntoFuture trait because Promise doesn't have a type argument
    // TODO explain more why we can't use the IntoFuture trait
    pub fn to_future< A >( &self ) -> PromiseFuture< A >
         where A: TryFrom< Value > + 'static,
               A::Error: Error {

        let ( sender, receiver ) = channel();

        self.done( |value| {
            // TODO is this correct ?
            match sender.send( value ) {
                Ok( _ ) => {},
                Err( _ ) => {},
            };
        } );

        PromiseFuture {
            future: Box::new( receiver.map_err( |x| JSError::new( x.description() ) ).and_then( future::result ) ),
            phantom: PhantomData,
        }
    }
}


pub struct PromiseFuture< A > {
    future: Box< Future< Item = A, Error = JSError > >,
    phantom: PhantomData< A >,
}


/*impl< A > PromiseFuture< A > {
    pub fn new< B >( callback: B ) -> Self
        where B: FnOnce( FnOnce( A ), FnOnce( JSError ) ) {
        js!( return new Promise( @{Once( callback )} ); ).try_from().unwrap()
    }
}*/


impl< A > std::fmt::Debug for PromiseFuture< A > {
    fn fmt( &self, formatter: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( formatter, "PromiseFuture" )
    }
}

impl< A > Future for PromiseFuture< A > {
    type Item = A;
    type Error = JSError;

    fn poll( &mut self ) -> Poll< Self::Item, Self::Error > {
        self.future.poll()
    }
}

impl< A > TryFrom< Value > for PromiseFuture< A >
    where A: TryFrom< Value > + 'static,
          A::Error: Error {

    type Error = ConversionError;

    fn try_from( v: Value ) -> Result< Self, Self::Error > {
        let promise: Promise = v.try_into()?;
        Ok( promise.to_future() )
    }
}