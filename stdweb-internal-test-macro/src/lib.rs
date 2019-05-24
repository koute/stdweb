extern crate proc_macro;

#[macro_use]
extern crate quote;

#[proc_macro_attribute]
pub fn test( _: proc_macro::TokenStream, input: proc_macro::TokenStream ) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        #[wasm_bindgen_test]
        #input
    };

    output.into()
}
