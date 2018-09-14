#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate base_x;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod macro_js_export;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn js_export( attrs: TokenStream, input: TokenStream ) -> TokenStream {
    macro_js_export::js_export( attrs, input )
}
