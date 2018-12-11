use syn;
use proc_macro2::{TokenStream, Span};
use sha1::Sha1;

use utils::{Target, dummy_idents};

fn hash( string: &str ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    format!( "{}", hasher.digest() )
}

pub fn js_shim_extern_code( target: Target, code: &str, arg_count: usize ) -> (syn::Ident, TokenStream) {
    let name = format!( "__cargo_web_snippet_{}", hash( code ) );
    let code_bytes = syn::LitByteStr::new( format!( "{}\0", code ).as_str().as_bytes(), Span::call_site() );

    let shim_name = syn::Ident::new( &name, Span::call_site() );
    let shim_args: Vec< _ > = dummy_idents( arg_count ).map( |name| quote! { #name: *const u8 } ).collect();
    let shim_args = &shim_args;
    let shim_args_passthrough: Vec< _ > = dummy_idents( arg_count ).map( |name| quote! { #name } ).collect();
    let shim_args_passthrough = &shim_args_passthrough;

    let asm_fn_name = syn::Ident::new( &format!( "__js_{}", arg_count ), Span::call_site() );
    let asm_fn_name = &asm_fn_name;

    let output = match target {
        Target::Emscripten => { quote! {
            const SNIPPET: &'static [u8] = #code_bytes;

            fn #shim_name( #(#shim_args),* ) -> i32 {
                extern "C" {
                    pub fn emscripten_asm_const_int( code: *const u8, ... ) -> i32;
                }
                unsafe {
                    emscripten_asm_const_int( SNIPPET as *const _ as *const u8, #(#shim_args_passthrough),* )
                }
            }
        }},
        Target::NativeWebAssembly => { quote! {
            const SNIPPET: &'static [u8] = #code_bytes;

            fn #shim_name( #(#shim_args),* ) -> i32 {
                extern "C" {
                    pub fn #asm_fn_name( #(#shim_args,)* code: *const u8 ) -> i32;
                }
                unsafe {
                    #asm_fn_name( #(#shim_args_passthrough,)* SNIPPET as *const _ as *const u8 )
                }
            }
        }}
    };

    (shim_name, output)
}
