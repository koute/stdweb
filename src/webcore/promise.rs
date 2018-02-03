use std;
use std::marker::PhantomData;
use {Value, Once};
use unstable::{TryInto, TryFrom};
use webcore::value::ConversionError;
use web::error::Error as JSError;
use std::error::Error;
use futures::{future, Future, Poll};
use futures::sync::oneshot::channel;


// TODO split this into Promise and PromiseFuture
pub struct PromiseFuture< A > {
    promise: Value,
    future: Box< Future< Item = A, Error = JSError > >,
    phantom: PhantomData< A >,
}


impl< A > std::fmt::Debug for PromiseFuture< A > {
    fn fmt( &self, formatter: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( formatter, "PromiseFuture {:?}", self.promise )
    }
}


impl< A > Future for PromiseFuture< A > {
    type Item = A;
    type Error = JSError;

    fn poll (&mut self ) -> Poll< Self::Item, Self::Error > {
        self.future.poll()
    }
}


// TODO this should probably check instanceof Promise
impl< A: TryFrom< Value > > TryFrom< Value > for PromiseFuture< A > where A: 'static, A::Error: Error {
    type Error = ConversionError;

    fn try_from( v: Value ) -> Result< Self, Self::Error > {
        match v {
            Value::Reference( ref r ) => {
                let ( sender, receiver ) = channel();

                let callback = |value: Value, success: bool| {
                    let value: Result< A, JSError > = if success {
                        let value: Result< A, A::Error > = value.try_into();
                        value.map_err( |e| JSError::new( e.description() ) )
                    } else {
                        let value: Result< JSError, ConversionError > = value.try_into();
                        value.map_err( |e| JSError::new( e.description() ) ).and_then( Err )
                    };

                    // TODO is this correct ?
                    match sender.send( value ) {
                        Ok( _ ) => {},
                        Err( _ ) => {},
                    };
                };

                Ok( PromiseFuture {
                    promise: js! {
                        var callback = @{Once( callback )};

                        return @{r}.then( function (value) {
                            callback( value, true );
                        }, function (value) {
                            callback( value, false );
                        } );
                    },
                    future: Box::new( receiver.map_err( |x| JSError::new( x.description() ) ).and_then( future::result ) ),
                    phantom: PhantomData,
                } )
            },
            other => Err( ConversionError::Custom( format!( "Expected Promise but got: {:?}", other ) ) ),
        }
    }
}