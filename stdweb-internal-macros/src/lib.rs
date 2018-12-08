#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate base_x;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha1;

#[cfg(test)]
mod testutils;

mod utils;

mod macro_js_export;
mod macro_async_test;
mod macro_js_raw;
mod macro_js;

mod attr_hack;
mod js_stringify;
mod js_shim;

fn emit( result: syn::parse::Result< proc_macro2::TokenStream > ) -> proc_macro::TokenStream {
    match result {
        Ok( stream ) => stream.into(),
        Err( error ) => proc_macro::TokenStream::from( error.to_compile_error() )
    }
}

#[proc_macro_attribute]
pub fn js_export( attrs: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    macro_js_export::js_export( attrs, input )
}

#[proc_macro_attribute]
pub fn async_test( attrs: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    macro_async_test::async_test( attrs, input )
}

#[proc_macro]
pub fn js_raw( input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    emit( macro_js_raw::js_raw( input.into() ) )
}

#[proc_macro_attribute]
pub fn js_raw_attr( _: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    emit( macro_js_raw::js_raw_attr( input.into() ) )
}

#[proc_macro_attribute]
pub fn js_attr( _: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    emit( macro_js::js_attr( input.into(), false ) )
}

#[proc_macro_attribute]
pub fn js_no_return_attr( _: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    emit( macro_js::js_attr( input.into(), true ) )
}
