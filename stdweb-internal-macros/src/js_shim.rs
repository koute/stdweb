use std::env;
use std::path::PathBuf;
use std::fs;
use std::fmt::Write;

use syn;
use proc_macro2::{TokenStream, Span};
use sha1::Sha1;

use utils::{Target, dummy_idents};

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

fn database_path() -> PathBuf {
    let target_path = env::var_os( "CARGO_WEB_TARGET_DIR" )
        .map( PathBuf::from )
        .expect( "you need to use `cargo-web` to compile your project for the `wasm32-unknown-unknown` target" );
    assert!( target_path.exists() );

    target_path.join( ".cargo-web" ).join( "snippets" )
}

fn output_snippet( snippet: &Snippet ) {
    let hash = hash( &snippet.name );
    let directory = database_path().join( &hash[ 0..2 ] );

    fs::create_dir_all( &directory ).expect( "failed to create a directory for the JS snippet database" );
    let path = directory.join( format!( "{}.json", hash ) );

    let blob: Vec< u8 > = serde_json::to_string( &snippet ).expect( "failed to convert the JS snipped to JSON" ).into_bytes();
    if path.exists() {
        if let Ok( size ) = path.metadata().map( |metadata| metadata.len() ) {
            if size == blob.len() as u64 {
                return;
            }
        }
    }

    fs::write( path, blob ).expect( "failed to write a JS snippet" );
}

pub fn js_shim_extern_code( target: Target, code: &str, arg_count: usize, return_ty: Option< syn::Type > ) -> (syn::Ident, TokenStream) {
    let snippet = Snippet {
        name: format!( "__cargo_web_snippet_{}", hash( code ) ),
        code: code.to_owned(),
        arg_count
    };

    let has_return_value = return_ty.is_some();
    let return_signature = if let Some( ty ) = return_ty {
        quote! { -> #ty }
    } else {
        quote! {}
    };

    let shim_name = syn::Ident::new( &snippet.name, Span::call_site() );
    let shim_args: Vec< _ > = dummy_idents( arg_count ).map( |name| quote! { #name: *const u8 } ).collect();
    let shim_args_passthrough: Vec< _ > = dummy_idents( arg_count ).map( |name| quote! { #name } ).collect();
    let output = match target {
        Target::Emscripten => {
            let code_bytes = syn::LitByteStr::new( format!( "{}\0", code ).as_str().as_bytes(), Span::call_site() );
            let return_semicolon = if has_return_value {
                quote! {}
            } else {
                quote! { ; }
            };

            quote! {
                const SNIPPET: &'static [u8] = #code_bytes;

                fn #shim_name( #(#shim_args),* ) #return_signature {
                    extern "C" {
                        pub fn emscripten_asm_const_int( code: *const u8, ... ) -> i32;
                    }

                    unsafe {
                        emscripten_asm_const_int( SNIPPET as *const _ as *const u8, #(#shim_args_passthrough),* ) #return_semicolon
                    }
                }
            }
        },
        Target::NativeWebAssembly => {
            output_snippet( &snippet );
            quote! {
                extern "C" {
                    pub fn #shim_name( #(#shim_args),* ) #return_signature;
                }
            }
        },
        Target::WasmBindgen => {
            let mut code_string = String::new();
            write!( &mut code_string, "export function {}(", &snippet.name ).unwrap();
            write!( &mut code_string, "Module" ).unwrap();
            if arg_count != 0 {
                write!( &mut code_string, ", " ).unwrap();
            }
            for nth in 0..arg_count {
                write!( &mut code_string, "${}", nth ).unwrap();
                if nth + 1 != arg_count {
                    write!( &mut code_string, ", " ).unwrap();
                }
            }
            write!( &mut code_string, ") {{ {} }}", code ).unwrap();
            let shim_name = &shim_name;
            let shim_args = &shim_args;
            quote! {
                use ::stdweb::private::wasm_bindgen::prelude::*;
                unsafe fn #shim_name( #(#shim_args),* ) #return_signature {
                    #[wasm_bindgen(inline_js = #code_string)]
                    extern "C" {
                        pub fn #shim_name( module: JsValue, #(#shim_args),* ) #return_signature;
                    }

                    #shim_name( ::stdweb::private::get_module(), #(#shim_args_passthrough),* )
                }
            }
        }
    };

    (shim_name, output)
}
