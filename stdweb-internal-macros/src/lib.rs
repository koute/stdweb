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

#[cfg(rust_1_30_or_newer)]
mod macro_js_export;
#[cfg(rust_1_30_or_newer)]
mod macro_async_test;
#[cfg(rust_1_30_or_newer)]
mod macro_js;

#[cfg(rust_1_30_or_newer)]
#[proc_macro_attribute]
pub fn js_export( attrs: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    macro_js_export::js_export( attrs, input )
}

#[cfg(rust_1_30_or_newer)]
#[proc_macro_attribute]
pub fn async_test( attrs: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    macro_async_test::async_test( attrs, input )
}

#[cfg(rust_1_30_or_newer)]
#[proc_macro]
pub fn js_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as macro_js::Js);
    let expanded = macro_js::transform_js(input);
    println!("{}", expanded);
    proc_macro::TokenStream::from(expanded)
}
