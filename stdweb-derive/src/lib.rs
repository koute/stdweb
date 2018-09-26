#![recursion_limit="512"]
#![feature(proc_macro_non_items)]

extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::DeriveInput;

mod reference_type;
mod js;

fn get_meta_items( attr: &syn::Attribute ) -> Option< Vec< syn::NestedMeta > > {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "reference" {
        match attr.interpret_meta() {
            Some( syn::Meta::List( meta ) ) => Some( meta.nested.into_iter().collect() ),
            _ => {
                panic!( "Unrecognized meta item type!" );
            }
        }
    } else {
        None
    }
}

/// A derive macro for defining custom reference types.
///
/// For example:
///
/// ```rust
/// #[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
/// #[reference(instance_of = "Error")]
/// pub struct Error( Reference );
///
/// #[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
/// #[reference(instance_of = "TypeError")]
/// #[reference(subclass_of(Error))]
/// pub struct TypeError( Reference );
/// ```
///
/// And then you can do:
///
/// ```rust
/// // You can use `try_into` to cast a `Value` to your type.
/// let error: TypeError = js!( return new TypeError(); ).try_into().unwrap();
///
/// // You can also pass your type freely into the `js!` macro:
/// js!( console.log( @{error} ); );
/// ```
#[proc_macro_derive(ReferenceType, attributes(reference))]
pub fn derive_reference_type( input: TokenStream ) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = reference_type::impl_reference(input);
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn js_new(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as js::Js);
    let expanded = js::transform_js(input);
    println!("{}", expanded);
    TokenStream::from(expanded)
}
