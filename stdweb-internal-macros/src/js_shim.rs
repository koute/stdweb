use syn;
use proc_macro2::{TokenStream, Span};
use sha1::Sha1;

use utils::dummy_idents;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Snippet {
    name: String,
    code: String,
    arg_count: usize
}

fn hash( string: &str ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    format!( "{}", hasher.digest() )
}

pub fn js_shim_extern_code( code: &str, arg_count: usize ) -> (syn::Ident, TokenStream) {
    let snippet = Snippet {
        name: format!( "__cargo_web_snippet_{}", hash( code ) ),
        code: code.to_owned(),
        arg_count
    };

    let blob: Vec< u8 > = serde_json::to_string( &snippet ).unwrap().into_bytes();
    let blob_length = blob.len();

    let code_bytes = syn::LitByteStr::new( format!( "{}\0", code ).as_str().as_bytes(), Span::call_site() );

    let shim_name = syn::Ident::new( &snippet.name, Span::call_site() );
    let shim_args: Vec< _ > = dummy_idents( arg_count ).map( |name| quote! { #name: *const u8 } ).collect();
    let shim_args = &shim_args;
    let shim_args_passthrough = dummy_idents( arg_count ).map( |name| quote! { #name } );
    let output = quote! {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        #[link_section = "cargo-web-js"]
        static SNIPPET: [u8; #blob_length] = [#(#blob),*];

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        extern "C" {
            pub fn #shim_name( #(#shim_args),* ) -> i32;
        }

        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        const SNIPPET: &'static [u8] = #code_bytes;

        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        fn #shim_name( #(#shim_args),* ) -> i32 {
            extern "C" {
                pub fn emscripten_asm_const_int( code: *const u8, ... ) -> i32;
            }
            unsafe {
                emscripten_asm_const_int( SNIPPET as *const _ as *const u8, #(#shim_args_passthrough),* )
            }
        }
    };

    (shim_name, output)
}
