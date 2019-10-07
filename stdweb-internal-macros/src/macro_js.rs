use std::fmt::Write;
use syn;
use syn::parse::Result;
use proc_macro2::TokenStream;

use attr_hack::AttrHack;
use js_shim::js_shim_extern_code;
use js_stringify::StringifiedCode;
use utils::{Target, dummy_idents};

// TODO: Delete this once expression procedural macros are stable.
pub fn js_attr( target: Target, input: TokenStream, outer_no_return: bool ) -> Result< TokenStream > {
    let wrapper: AttrHack< StringifiedCode > = syn::parse2( input )?;
    let wrapper_name = wrapper.fn_name;
    let snippet = wrapper.inner;
    let inner_no_return = outer_no_return || snippet.code( 0 ).contains( "return" ) == false;
    let inner_arg_count = snippet.arg_count() + if inner_no_return { 0 } else { 1 };
    let outer_arg_count = snippet.arg_count() + if outer_no_return { 0 } else { 1 };

    let initial_arg_index = if inner_no_return { 0 } else { 1 };
    let mut code = snippet.code( initial_arg_index );
    if !inner_no_return {
        code = format!( "Module.STDWEB_PRIVATE.from_js($0, (function(){{{}}})());", code );
    }

    let mut prelude = String::new();
    for nth in initial_arg_index..inner_arg_count {
        write!( prelude, "${} = Module.STDWEB_PRIVATE.to_js(${});", nth, nth ).unwrap();
    }

    code = format!( "{}{}", prelude, code );

    let (shim_name, shim) = js_shim_extern_code( target, &code, inner_arg_count, wrapper.return_ty );

    let arg_names: Vec< _ > = dummy_idents( outer_arg_count ).collect();
    let prototype_args = arg_names.clone().into_iter().map( |name| quote! { #name: *const u8 } );

    let call_args: Vec< _ >;
    if inner_no_return && !outer_no_return {
        call_args = arg_names.into_iter().skip( 1 ).collect();
    } else {
        call_args = arg_names.into_iter().collect();
    }

    let call_args = call_args.into_iter().map( |name| quote! { #name } );
    let output = quote! {
        fn #wrapper_name( #(#prototype_args),* ) {
            #shim
            unsafe {
                #shim_name( #(#call_args),* )
            }
        }
    };

    Ok( output )
}
