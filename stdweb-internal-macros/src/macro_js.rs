use std::fmt::Write;
use syn;
use syn::parse::Result;
use proc_macro2::TokenStream;

use attr_hack::AttrHack;
use js_shim::js_shim_extern_code;
use js_stringify::StringifiedCode;
use utils::dummy_idents;

// TODO: Delete this once expression procedural macros are stable.
pub fn js_attr( input: TokenStream, no_return: bool ) -> Result< TokenStream > {
    let wrapper: AttrHack< StringifiedCode > = syn::parse2( input )?;
    let wrapper_name = wrapper.fn_name;
    let snippet = wrapper.inner;
    let mut arg_count = snippet.arg_count();
    if !no_return {
        arg_count += 1;
    }

    let initial_arg_index = if no_return { 0 } else { 1 };
    let mut code = snippet.code( initial_arg_index );
    if !no_return {
        code = format!( "Module.STDWEB_PRIVATE.from_js($0, (function(){{{}}})());", code );
    }

    let mut prelude = String::new();
    for nth in initial_arg_index..arg_count {
        write!( prelude, "${} = Module.STDWEB_PRIVATE.to_js(${});", nth, nth ).unwrap();
    }

    code = format!( "{}{}", prelude, code );

    let (shim_name, shim) = js_shim_extern_code( &code, arg_count );

    let prototype_args = dummy_idents( arg_count ).map( |name| quote! { #name: *const u8 } );
    let call_args = dummy_idents( arg_count ).map( |name| quote! { #name } );
    let output = quote! {
        fn #wrapper_name( #(#prototype_args),* ) -> i32 {
            #shim
            unsafe {
                #shim_name( #(#call_args),* )
            }
        }
    };

    Ok( output )
}
