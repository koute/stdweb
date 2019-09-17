use syn;
use proc_macro::TokenStream;
use proc_macro2::{self, Span};

#[cfg(test)]
use testutils::assert_code_eq;

fn check_return_type( return_type: &syn::ReturnType ) -> bool {
    match return_type {
        &syn::ReturnType::Default => true,
        &syn::ReturnType::Type( _, ref ty ) => {
            match **ty {
                syn::Type::Tuple( ref tuple ) if tuple.elems.is_empty() => true,
                _ => false
            }
        }
    }
}

enum TestKind {
    Simple {
        callback_ident: syn::Ident
    },
    Fallible {
        callback_ident: syn::Ident,
        result_ty: syn::Type
    },
    Synchronous
}

// TODO: There must be a cleaner way to do this.
fn check_decl( decl: &syn::Signature ) -> TestKind {
    assert!( decl.generics.lifetimes().next().is_none(), "Lifetimes are yet not supported" );
    assert!( decl.generics.where_clause.is_none(), "`where` clauses are not supported" );
    assert!( decl.variadic.is_none(), "Variadic functions are not supported" );

    if !check_return_type( &decl.output ) {
        panic!( "The function should not return anything!" );
    }

    let mut type_params: Vec< _ > = decl.generics.type_params().collect();
    if type_params.is_empty() && decl.inputs.is_empty() {
        // Exactly like a normal #[test].
        //   fn test_foo() {}
        return TestKind::Synchronous;
    }

    assert_eq!( type_params.len(), 1, "The function should have a single type parameter" );

    let type_param = type_params.pop().unwrap();
    assert!( type_param.attrs.is_empty(), "Type param attributes are not supported" );
    assert!( type_param.default.is_none(), "Type param defaults are not supported" );
    assert!( type_param.eq_token.is_none() );
    assert_eq!( type_param.bounds.len(), 1, "The type param should have only one bound" );
    let bound = match type_param.bounds[ 0 ].clone() {
        syn::TypeParamBound::Lifetime( .. ) => panic!( "Lifetime type param bounds are not supported" ),
        syn::TypeParamBound::Trait( bound ) => bound
    };
    match bound.modifier {
        syn::TraitBoundModifier::None => {},
        syn::TraitBoundModifier::Maybe( _ ) => panic!( "'?Trait' type bounds are not supported" )
    }
    assert!( bound.lifetimes.is_none(), "Lifetimes in type param bounds are not supported" );

    if !bound.path.leading_colon.is_none() ||
        bound.path.segments.len() != 1 ||
        bound.path.segments[ 0 ].ident != "FnOnce"
    {
        panic!( "Unsupported type bound" );
    }

    enum Kind {
        Simple,
        Fallible {
            result_ty: syn::Type
        }
    }

    let kind: Kind = match bound.path.segments[ 0 ].arguments {
        syn::PathArguments::Parenthesized( syn::ParenthesizedGenericArguments { ref inputs, ref output, .. } ) => {
            match output {
                syn::ReturnType::Default => {},
                _ => panic!( "Unsupported type bound" )
            }

            let inputs: Vec< _ > = inputs.iter().collect();
            match *inputs {
                [] => {
                    // A test which can only succeed, or timeout.
                    //  fn test_foo< F: FnOnce() >( cb: F ) {}
                    Kind::Simple
                },
                [
                    syn::Type::Path(
                        syn::TypePath {
                            qself: None,
                            path: syn::Path {
                                leading_colon: None,
                                segments
                            }
                        }
                    )
                ] if segments.len() == 1 => {
                    let segment = &segments[ 0 ];
                    if segment.ident != "Result" {
                        panic!( "Unsupported type bound" );
                    }

                    match segment.arguments {
                        syn::PathArguments::AngleBracketed( ref args ) => {
                            if args.args.len() != 2 {
                                panic!( "Unsupported type bound" );
                            }
                            match args.args[ 0 ] {
                                syn::GenericArgument::Type(
                                    syn::Type::Tuple( syn::TypeTuple { ref elems, .. } )
                                ) if elems.is_empty() => {
                                    // A test which can suceed, fail or timeout.
                                    //  fn test_foo< F: FnOnce( Result< (), E > ) >( cb: F ) {}
                                    Kind::Fallible {
                                        result_ty: inputs[ 0 ].clone()
                                    }
                                },
                                _ => panic!( "Unsupported type bound" )
                            }
                        },
                        _ => panic!( "Unsupported type bound" )
                    }
                },
                _ => panic!( "Unsupported type bound" )
            }
        },
        _ => panic!( "Unsupported type bound" )
    };

    if decl.inputs.len() != 1 {
        panic!( "Expected a function with a single argument!" );
    }

    let arg = decl.inputs.last().unwrap();
    match *arg {
        syn::FnArg::Receiver( .. ) => panic!( "`self` is not supported" ),
        syn::FnArg::Typed( syn::PatType { ref pat, ref ty, .. } ) => {
            match **pat {
                syn::Pat::Ident( ref pat ) => {
                    assert!( pat.by_ref.is_none(), "`ref` bindings are not supported" );
                    assert!( pat.mutability.is_none(), "`mut` bindings are not supported" );
                    assert!( pat.subpat.is_none(), "Subpatterns are not supported" );

                    match **ty {
                        syn::Type::Path(
                            syn::TypePath {
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    ref segments
                                }
                            }
                        ) if segments.len() == 1 => {
                            if type_param.ident != segments[ 0 ].ident {
                                panic!( "Unsupported argument type" );
                            }
                        },
                        _ => panic!( "Unsupported argument type" )
                    }

                    let callback_ident = pat.ident.clone();
                    match kind {
                        Kind::Simple => TestKind::Simple { callback_ident },
                        Kind::Fallible { result_ty } => TestKind::Fallible {
                            callback_ident,
                            result_ty
                        }
                    }
                },
                _ => panic!( "Argument patterns are not supported" )
            }
        }
    }
}

fn async_test_impl( item: syn::Item ) -> proc_macro2::TokenStream {
    let (ident, block, test_kind) = match item {
        syn::Item::Fn( function ) => {
            let test_kind = check_decl( &function.sig );
            (function.sig.ident.clone(), function.block, test_kind)
        },
        _ => panic!( "`#[async_test]` attached to an unsupported element!" )
    };

    let inner;
    match test_kind {
        TestKind::Simple { callback_ident } => {
            let prelude = quote! {
                let #callback_ident = {
                    let resolve = js!( return ASYNC_TEST_PRIVATE.resolve; );
                    move || {
                        js!(
                            @{resolve}();
                        );
                    }
                };
            };

            inner = quote! {
                #prelude
                #block
            };
        },
        TestKind::Fallible { callback_ident, result_ty } => {
            let prelude = quote! {
                let #callback_ident = {
                    let resolve = js!( return ASYNC_TEST_PRIVATE.resolve; );
                    let reject = js!( return ASYNC_TEST_PRIVATE.reject; );
                    move |result: #result_ty| {
                        match result {
                            Ok(()) => js! {
                                @{resolve}();
                            },
                            Err( error ) => js! {
                                @{reject}(@{format!( "{:?}", error )});
                            }
                        };
                    }
                };
            };

            inner = quote! {
                #prelude
                #block
            };
        },
        TestKind::Synchronous => {
            inner = quote! {
                (move || {
                    #block
                })();

                js! {
                    ASYNC_TEST_PRIVATE.resolve();
                };
            }
        }
    }

    let symbol = syn::Ident::new( &format!( "__async_test__{}", ident ), Span::call_site() );
    let output = quote! {
        #[cfg(test)]
        #[linkage = "external"]
        #[no_mangle]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        fn #symbol() {
            #inner
        }
    };

    output
}

pub fn async_test( attrs: TokenStream, input: TokenStream ) -> TokenStream {
    if !attrs.is_empty() {
        panic!( "Extra attributes are not supported in `#[async_test]`!" );
    }

    let input: proc_macro2::TokenStream = input.into();
    let item: syn::Item = syn::parse2( input ).unwrap();

    async_test_impl( item ).into()
}

#[test]
fn test_async_test_simple() {
    let input = quote! {
        fn foobar< F: FnOnce() >( done: F ) {
            if true {
                done();
            }
        }
    };

    let expected = quote! {
        #[cfg(test)]
        #[linkage = "external"]
        #[no_mangle]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        fn __async_test__foobar() {
            let done = {
                let resolve = js ! ( return ASYNC_TEST_PRIVATE . resolve ; );
                move || {
                    js ! ( @ { resolve } ( ) ; );
                }
            };
            {
                if true {
                    done();
                }
            }
        }
    };

    let output = async_test_impl( syn::parse2( input ).unwrap() );
    assert_code_eq( output, expected );
}

#[test]
fn test_async_test_fallible() {
    let input = quote! {
        #[async_test]
        fn foobar< F: FnOnce( Result< (), i32 > ) >( done: F ) {
            done( Ok(()) );
        }
    };

    let expected = quote! {
        #[cfg(test)]
        #[linkage = "external"]
        #[no_mangle]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        fn __async_test__foobar() {
            let done = {
                let resolve = js ! ( return ASYNC_TEST_PRIVATE . resolve ; );
                let reject = js ! ( return ASYNC_TEST_PRIVATE . reject ; );
                move |result: Result<(), i32>| { match result {
                    Ok(()) => js ! { @ { resolve } ( ) ; },
                    Err(error) => js ! { @ { reject } ( @ { format ! ( "{:?}" , error ) } ) ; }
                };}
            };
            {
                done(Ok(()));
            }
        }
    };

    let output = async_test_impl( syn::parse2( input ).unwrap() );
    assert_code_eq( output, expected );
}

#[test]
fn test_async_test_synchronous() {
    let input = quote! {
        fn foobar() {
            body();
        }
    };

    let expected = quote! {
        #[cfg(test)]
        #[linkage = "external"]
        #[no_mangle]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        fn __async_test__foobar() {
            (move || {{
                body();
            }})();
            js ! { ASYNC_TEST_PRIVATE . resolve ( ) ; };
        }
    };

    let output = async_test_impl( syn::parse2( input ).unwrap() );
    assert_code_eq( output, expected );
}
