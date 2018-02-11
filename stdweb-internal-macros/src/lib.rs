#![feature(proc_macro)]
#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate base_x;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use proc_macro::TokenStream;

#[derive(Clone, Serialize, Deserialize, Debug)]
enum TypeMetadata {
    I32,
    F64,
    Custom {
        name: Option< String >,
        conversion_fn: String
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ArgMetadata {
    name: String,
    ty: TypeMetadata
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ExportMetadata {
    name: String,
    args: Vec< ArgMetadata >,
    result: Option< TypeMetadata >
}

// This is a base62 encoding which consists of only alpha-numeric characters.
// Generated with: (('A'..'Z').to_a + ('a'..'z').to_a + ('0'..'9').to_a).join("")
const ENCODING_BASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn match_shallow_path( path: &syn::Path ) -> Option< &str > {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return None;
    }

    let segment = &path.segments[ 0 ];
    match &segment.arguments {
        &syn::PathArguments::None => {},
        _ => return None
    }

    Some( segment.ident.as_ref() )
}

fn match_type( ty: &syn::Type ) -> ExportType {
    match ty {
        &syn::Type::Reference( ref ty ) => {
            assert!( ty.mutability.is_none(), "`mut` bindings are not supported" );
            match *ty.elem {
                syn::Type::Path( ref path ) if match_shallow_path( &path.path ).map( |path| path == "str" ).unwrap_or( false ) => {
                    ExportType::StrRef
                },
                syn::Type::Slice( ref slice ) => {
                    ExportType::Slice( (*slice.elem).clone() )
                },
                ref elem => ExportType::UnknownRef( elem.clone() )
            }
        },
        &syn::Type::Path( ref path ) => {
            let name = match match_shallow_path( &path.path ) {
                Some( name ) => name,
                None => return ExportType::Unknown( ty.clone() )
            };

            match name {
                "i32" => ExportType::I32,
                "f64" => ExportType::F64,
                _ => ExportType::Unknown( ty.clone() )
            }
        },
        &syn::Type::Tuple( ref tuple ) if tuple.elems.is_empty() => ExportType::Unit,
        _ => ExportType::Unknown( ty.clone() )
    }
}

enum ExportType {
    Unit,
    I32,
    F64,
    StrRef,
    Slice( syn::Type ),
    Unknown( syn::Type ),
    UnknownRef( syn::Type )
}

struct ExportArg {
    ident: syn::Ident,
    ty: ExportType
}

struct Export {
    ident: syn::Ident,
    return_ty: ExportType,
    args: Vec< ExportArg >
}

fn process( exports: Vec< Export > ) -> quote::Tokens {
    let mut output = Vec::new();
    for export in exports {
        let export_result;
        let export_result_conversion;
        let export_result_metadata;
        let mut export_args = Vec::new();
        let mut export_args_metadata = Vec::new();
        let mut export_args_idents = Vec::new();
        let mut export_args_conversions = Vec::new();

        match export.return_ty {
            ExportType::Unit => {
                export_result = quote! { () };
                export_result_conversion = quote! {};
                export_result_metadata = None;
            },
            ExportType::I32 => {
                export_result = quote! { i32 };
                export_result_conversion = quote! {};
                export_result_metadata = Some( TypeMetadata::I32 );
            },
            ExportType::F64 => {
                export_result = quote! { f64 };
                export_result_conversion = quote! {};
                export_result_metadata = Some( TypeMetadata::F64 );
            },
            ExportType::Unknown( _ ) |
            ExportType::UnknownRef( _ ) |
            ExportType::StrRef |
            ExportType::Slice( _ ) => {
                // TODO: For known types generate more efficient serialization.
                export_result = quote! { () };
                // TODO: Figure out a better way to do this, if possible.
                export_result_conversion = quote! {
                    let __result = ::stdweb::private::IntoNewtype::into_newtype( __result );
                    let __result_memory_required = ::stdweb::private::JsSerializeOwned::memory_required_owned( &__result );
                    let mut __result_arena = ::stdweb::private::PreallocatedArena::new( __result_memory_required );
                    let mut __result = Some( __result );
                    let __result = ::stdweb::private::JsSerializeOwned::into_js_owned( &mut __result, &mut __result_arena );
                    let __result = &__result as *const _;
                    __js_raw_asm!( "Module.STDWEB_PRIVATE.tmp = Module.STDWEB_PRIVATE.to_js( $0 );", __result );
                    let __result = ();
                };
                export_result_metadata = Some( TypeMetadata::Custom {
                    name: None,
                    conversion_fn: "Module.STDWEB_PRIVATE.acquire_tmp".to_owned()
                });
            }
        }

        for arg in &export.args {
            let export_arg_ident = arg.ident.clone();
            let export_arg_ty;
            let export_arg_ty_metadata;
            match arg.ty {
                ExportType::I32 => {
                    export_arg_ty = quote! { i32 };
                    export_arg_ty_metadata = TypeMetadata::I32;
                },
                ExportType::F64 => {
                    export_arg_ty = quote! { f64 };
                    export_arg_ty_metadata = TypeMetadata::F64;
                },
                ExportType::Unit => {
                    panic!( "Receiving arguments of type `()` isn't supported" );
                },
                ExportType::Unknown( _ ) |
                ExportType::UnknownRef( _ ) |
                ExportType::StrRef |
                ExportType::Slice( _ ) => {
                    // TODO: For known types generate more efficient serialization.
                    export_arg_ty = quote! { i32 };
                    export_args_conversions.push( quote! {
                        let #export_arg_ident = {
                            let pointer = #export_arg_ident as *mut ::stdweb::private::SerializedValue;
                            unsafe {
                                let value = (&*pointer).deserialize();
                                ::stdweb::private::__web_free( pointer as *mut u8, ::std::mem::size_of::< ::stdweb::private::SerializedValue >() );
                                value
                            }
                        };
                    });

                    export_arg_ty_metadata = TypeMetadata::Custom {
                        name: None,
                        conversion_fn: "Module.STDWEB_PRIVATE.prepare_any_arg".to_owned()
                    };
                }
            }

            export_args_idents.push( export_arg_ident.clone() );
            export_args_metadata.push( ArgMetadata {
                name: format!( "{}", export_arg_ident ),
                ty: export_arg_ty_metadata
            });
            export_args.push( quote! {
                #export_arg_ident: #export_arg_ty
            });
        }

        for arg in &export.args {
            let export_arg_ident = arg.ident.clone();
            match arg.ty {
                // TODO: Throw a JS exception if `try_into` fails.
                ExportType::Unknown( ref ty ) => {
                    let ty = ty.clone();
                    export_args_conversions.push( quote! {
                        let #export_arg_ident: #ty = #export_arg_ident.try_into().unwrap();
                    });
                },
                ExportType::StrRef => {
                    export_args_conversions.push( quote! {
                        let #export_arg_ident: String = #export_arg_ident.try_into().unwrap();
                        let #export_arg_ident: &str = &#export_arg_ident;
                    });
                },
                ExportType::Slice( ref ty ) => {
                    export_args_conversions.push( quote! {
                        let #export_arg_ident: Vec< #ty > = #export_arg_ident.try_into().unwrap();
                        let #export_arg_ident: &[#ty] = &#export_arg_ident;
                    });
                },
                ExportType::UnknownRef( ref ty ) => {
                    let ty = ty.clone();
                    export_args_conversions.push( quote! {
                        let #export_arg_ident: #ty = #export_arg_ident.try_into().unwrap();
                        let #export_arg_ident = &#export_arg_ident;
                    });
                },
                _ => {}
            }
        }

        let metadata = ExportMetadata {
            name: format!( "{}", export.ident ),
            args: export_args_metadata,
            result: export_result_metadata
        };

        let json_metadata = serde_json::to_string( &metadata ).unwrap();
        let encoded_metadata = base_x::encode( ENCODING_BASE, json_metadata.as_bytes() );
        let export_ident = syn::Ident::from( format!( "__JS_EXPORT_{}", &encoded_metadata ) );
        let original_ident = export.ident.clone();

        output.push(
            quote! {
                #[doc(hidden)]
                #[no_mangle]
                #[deny(private_no_mangle_fns)]
                #[allow(unused_imports)]
                pub extern fn #export_ident( #(#export_args),* ) -> #export_result {
                    use ::stdweb::unstable::TryInto;
                    #(#export_args_conversions)*
                    let __result = #original_ident( #(#export_args_idents),* );
                    #export_result_conversion
                    return __result;
                }
            }
        );
    }

    quote! { #(#output)* }
}

fn into_export( ident: syn::Ident, decl: &syn::FnDecl ) -> Export {
    assert!( decl.generics.lifetimes().next().is_none(), "Lifetimes are not yet not supported" );
    assert!( decl.generics.type_params().next().is_none(), "Generics are not supported" );
    assert!( decl.generics.where_clause.is_none(), "`where` clauses are not supported" );
    assert!( decl.variadic.is_none(), "Variadic functions are not supported" );

    let return_ty = match &decl.output {
        &syn::ReturnType::Default => ExportType::Unit,
        &syn::ReturnType::Type( _, ref ty ) => match_type( ty )
    };

    let mut args = Vec::new();
    for (index, arg) in decl.inputs.iter().cloned().enumerate() {
        match arg {
            syn::FnArg::SelfRef( .. ) => panic!( "`&self` is not supported" ),
            syn::FnArg::SelfValue( .. ) => panic!( "`self` is not supported" ),
            syn::FnArg::Ignored( ty ) => {
                let ident = syn::Ident::from( format!( "__arg_{}", index ) );
                args.push( ExportArg {
                    ident,
                    ty: match_type( &ty )
                });
            },
            syn::FnArg::Captured( cap ) => {
                match cap.pat {
                    syn::Pat::Wild( _ ) => {
                        let ident = syn::Ident::from( format!( "__arg_{}", index ) );
                        args.push( ExportArg {
                            ident,
                            ty: match_type( &cap.ty )
                        });
                    },
                    syn::Pat::Ident( pat ) => {
                        assert!( pat.by_ref.is_none(), "`ref` bindings are not supported" );
                        assert!( pat.mutability.is_none(), "`mut` bindings are not supported" );
                        assert!( pat.subpat.is_none(), "Subpatterns are not supported" );

                        args.push( ExportArg {
                            ident: pat.ident,
                            ty: match_type( &cap.ty )
                        });
                    },
                    _ => panic!( "Argument patterns are not supported" )
                }
            },
            syn::FnArg::Inferred( _ ) => panic!( "inferred argument types are not supported" )
        }
    }

    Export {
        ident,
        return_ty,
        args
    }
}

#[proc_macro_attribute]
pub fn js_export( _: TokenStream, input: TokenStream ) -> TokenStream {
    let item: syn::Item = syn::parse( input ).unwrap();
    let mut exports = Vec::new();

    match item {
        syn::Item::Fn( ref function ) => {
            exports.push( into_export( function.ident.clone(), &function.decl ) );
        },
        _ => panic!( "`#[js_export]` attached to an unsupported element!" )
    }

    let generated = process( exports );
    let output = quote! {
        #item
        #generated
    };

    output.into()
}
