use proc_macro2::TokenStream;
use syn;
use syn::parse::{ParseStream, Parse, Result};

use attr_hack::AttrHack;
use js_shim::js_shim_extern_code;
use utils::{Target, dummy_idents};

#[cfg(test)]
use testutils::assert_code_eq;

fn parse_js_raw( input: TokenStream ) -> Result< JsRawInvocation > {
    syn::parse2( input )
}

struct JsRawCode( String );
struct JsRawInvocation {
    code: String,
    args: Vec< syn::Expr >
}

impl Parse for JsRawCode {
    fn parse( input: ParseStream ) -> Result< Self > {
        if let Ok( ident ) = syn::Ident::parse( input ) {
            if ident == "stringify" {
                input.parse::< Token![!] >()?;

                let inner;
                parenthesized!( inner in input );
                let code = inner.parse::< TokenStream >()?.to_string();
                return Ok( JsRawCode( code ) );
            } else if ident == "concat" {
                input.parse::< Token![!] >()?;

                let inner;
                parenthesized!( inner in input );
                let code: syn::punctuated::Punctuated< JsRawCode, Token![,] > = inner.parse_terminated( JsRawCode::parse )?;
                let code: Vec< String > = code.into_iter().map( |chunk| chunk.0 ).collect();
                let code = code.join( "" );
                return Ok( JsRawCode( code ) );
            } else {
                let ident_str = ident.to_string();
                return Err( syn::Error::new_spanned( ident, format!( "unexpected ident '{}'", ident_str ) ) );
            }
        }

        let literal: syn::LitStr = Parse::parse( input )?;
        let code = literal.value();
        Ok( JsRawCode( code ) )
    }
}

impl Parse for JsRawInvocation {
    fn parse( input: ParseStream ) -> Result< Self > {
        let code = input.parse::< JsRawCode >()?.0;
        let mut args = Vec::new();
        while !input.is_empty() {
            syn::token::Comma::parse( input )?;
            if input.is_empty() {
                break;
            }

            let arg: syn::Expr = input.parse()?;
            args.push( arg );
        }

        Ok( JsRawInvocation { code, args } )
    }
}

fn js_raw_code( target: Target, js_raw: JsRawInvocation ) -> TokenStream {
    let (shim_name, shim) = js_shim_extern_code( target, &js_raw.code, js_raw.args.len(), None );
    let args = js_raw.args;

    quote! {{
        #shim
        unsafe {
            #shim_name( #((#args) as *const u8),* )
        }
    }}
}

pub fn js_raw( target: Target, input: TokenStream ) -> Result< TokenStream > {
    let args = parse_js_raw( input )?;
    Ok( js_raw_code( target, args ) )
}

// TODO: Delete this once expression procedural macros are stable.
pub fn js_raw_attr( target: Target, input: TokenStream ) -> Result< TokenStream > {
    let wrapper: AttrHack< JsRawInvocation > = syn::parse2( input )?;
    let wrapper_name = wrapper.fn_name;
    let js_raw = wrapper.inner;
    let return_ty = wrapper.return_ty;

    let (shim_name, shim) = js_shim_extern_code( target, &js_raw.code, js_raw.args.len(), return_ty.clone() );
    let return_signature = if let Some( ty ) = return_ty {
        quote! { -> #ty }
    } else {
        quote! {}
    };

    let prototype_args = dummy_idents( js_raw.args.len() ).map( |name| quote! { #name: *const u8 } );
    let call_args = dummy_idents( js_raw.args.len() ).map( |name| quote! { #name } );
    let output = quote! {
        fn #wrapper_name( #(#prototype_args),* ) #return_signature {
            #shim
            unsafe {
                #shim_name( #(#call_args),* )
            }
        }
    };

    Ok( output )
}

#[test]
fn test_parse_js_raw_only_code() {
    let input = quote! {
        "function();"
    };

    let js_raw = parse_js_raw( input ).unwrap();
    assert_eq!( js_raw.code, "function();" );
    assert!( js_raw.args.is_empty() );
}

#[test]
fn test_parse_js_raw_one_simple_arg() {
    let input = quote! {
        "function( $0 );", arg
    };

    let js_raw = parse_js_raw( input ).unwrap();
    assert_eq!( js_raw.code, "function( $0 );" );
    assert_eq!( js_raw.args.len(), 1 );
    assert_code_eq( &js_raw.args[ 0 ], quote! { arg } );
}

#[test]
fn test_parse_js_raw_complex() {
    let input = quote! {
        "dummy", { struct Foobar {} &[Foobar, Foobar] }, 1234,
    };

    let js_raw = parse_js_raw( input ).unwrap();
    assert_eq!( js_raw.code, "dummy" );
    assert_eq!( js_raw.args.len(), 2 );
    assert_code_eq( &js_raw.args[ 0 ], quote! { { struct Foobar {} &[Foobar, Foobar] } } );
    assert_code_eq( &js_raw.args[ 1 ], quote! { 1234 } );
}

#[test]
fn test_parse_js_raw_stringify() {
    let input = quote! {
        stringify!( hello_world )
    };

    let js_raw = parse_js_raw( input ).unwrap();
    assert_eq!( js_raw.code, "hello_world" );
    assert!( js_raw.args.is_empty() );
}

#[test]
fn test_parse_js_raw_concat() {
    let input = quote! {
        concat!( "abc", "def", )
    };

    let js_raw = parse_js_raw( input ).unwrap();
    assert_eq!( js_raw.code, "abcdef" );
    assert!( js_raw.args.is_empty() );
}

#[test]
fn test_parse_js_raw_with_concat_and_stringify() {
    let input = quote! {
        concat!(
            "return foo( new ",
            stringify!( Bar ),
            "( baz )",
            " );"
        ),
        arg.as_ref().as_raw()
    };

    let js_raw = parse_js_raw( input ).unwrap();
    assert_eq!( js_raw.code, "return foo( new Bar( baz ) );" );
    assert_eq!( js_raw.args.len(), 1 );
    assert_code_eq( &js_raw.args[ 0 ], quote! { arg.as_ref().as_raw() } );
}

#[test]
fn test_js_raw_code_generation_succeeds() {
    let input = quote! {
        "function( $0 )",
        arg.as_ref().as_raw()
    };

    js_raw( Target::Emscripten, input ).unwrap();
}
