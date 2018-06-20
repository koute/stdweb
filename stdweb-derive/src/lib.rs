#![recursion_limit="512"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

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
    let input: DeriveInput = syn::parse( input ).unwrap();

    let name = input.ident;
    let generics_params = &input.generics.params;

    let mut instance_of = None;
    let mut subclass_of = Vec::new();
    let mut constraints = Vec::new();

    for meta_items in input.attrs.iter().filter_map( get_meta_items ) {
        for meta in meta_items {
            match meta {
                syn::NestedMeta::Meta( syn::Meta::NameValue( ref meta ) ) if meta.ident == "instance_of" => {
                    if instance_of.is_some() {
                        panic!( "Duplicate '#[reference(instance_of)]'!" );
                    }

                    if let syn::Lit::Str( ref str ) = meta.lit {
                        instance_of = Some( str.value() );
                    } else {
                        panic!( "The value of '#[reference(instance_of = ...)]' is not a string!" );
                    }
                },
                syn::NestedMeta::Meta( syn::Meta::NameValue( ref meta ) ) if meta.ident == "constraint" => {
                    if let syn::Lit::Str( ref str ) = meta.lit {
                        let val = str.value();
                        let parts : Vec<&str> = val.splitn(2,"=").collect();
                        if parts.len() != 2 {
                            panic!("The value of '#[reference(..., constraint = ...)]' must be in the form of \"js_field=value\"" );
                        }
                        constraints.push(( parts[0].to_owned(), parts[1].to_owned()) );
                    } else {
                        panic!( "The value of '#[reference(..., constraint = ...)]' is not a string!" );
                    }
                },
                syn::NestedMeta::Meta( syn::Meta::List( ref meta ) ) if meta.ident == "subclass_of" => {
                    for nested in &meta.nested {
                        match *nested {
                            syn::NestedMeta::Meta( ref nested ) => {
                                match *nested {
                                    syn::Meta::Word( ident ) => {
                                        subclass_of.push( ident );
                                    },
                                    _ => panic!( "The value of '#[reference(subclass_of(...))]' is invalid!" )
                                }
                            },
                            _ => panic!( "The value of '#[reference(subclass_of(...))]' is invalid!" )
                        }
                    }
                },
                syn::NestedMeta::Meta( ref meta ) => {
                    panic!( "Unrecognized attribute: '#[reference({})]'", meta.name() );
                },
                _ => panic!( "Unrecognized attribute!" )
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        syn::Data::Struct( ref data ) => {
            match data.fields {
                syn::Fields::Unnamed( ref fields ) => {
                    fn invalid_structure() {
                        panic!( "The structure can only have either (Reference) or (Reference, PhantomData)!" );
                    }

                    let fields = &fields.unnamed;
                    if fields.len() != 1 && fields.len() != 2 {
                        invalid_structure();
                    }

                    let mut fields_iter = fields.iter();
                    match fields_iter.next().unwrap().ty {
                        syn::Type::Path( ref ty_path ) => {
                            if ty_path.qself.is_some() {
                                invalid_structure();
                            }

                            let segs: Vec< _ > = ty_path.path.segments.iter().collect();
                            if segs.last().unwrap().ident != "Reference" || !segs.last().unwrap().arguments.is_empty() {
                                invalid_structure();
                            }

                            match segs.len() {
                                1 => {},
                                2 => {
                                    if segs.first().unwrap().ident != "stdweb" {
                                        invalid_structure();
                                    }
                                },
                                _ => invalid_structure()
                            }
                        },
                        _ => invalid_structure()
                    }

                    match fields_iter.next().map( |field| &field.ty ) {
                        Some( &syn::Type::Path( ref ty_path ) ) => {
                            if ty_path.qself.is_some() {
                                invalid_structure();
                            }

                            let segs: Vec< _ > = ty_path.path.segments.iter().collect();
                            if segs.last().unwrap().ident != "PhantomData" {
                                invalid_structure();
                            }
                        },
                        Some( _ ) => invalid_structure(),
                        None => {}
                    }
                },
                _ => panic!( "Only tuple structures are supported!" )
            }
        },
        _ => panic!( "Only tuple structures are supported!" )
    }

    let mut default_args = Vec::new();
    for param in input.generics.params.iter() {
        match *param {
            syn::GenericParam::Type( _ ) => {
                default_args.push( quote! {
                    ::std::default::Default::default()
                });
            },
            _ => {}
        }
    }
    let default_args = quote! { #(#default_args),* };

    let instance_of_impl = match instance_of {
        Some( js_name ) => {
            let mut js_constraints : Vec<String> = constraints.into_iter().map(|constraint|{
                let js_field = constraint.0.replace("\"", "\\\"");
                let js_value = constraint.1.replace("\"", "\\\"");
                format!("_ref[\"{}\"] === \"{}\"", &js_field, &js_value)
            }).collect();

            js_constraints.insert( 0, format!("_ref instanceof {}", &js_name) );

            let js_constraints = js_constraints.join(" && ");

            quote! {
                impl #impl_generics ::stdweb::InstanceOf for #name #ty_generics {
                    #[inline]
                    fn instance_of( reference: &::stdweb::Reference ) -> bool {
                        __js_raw_asm!(
                            concat!( "var _ref = Module.STDWEB_PRIVATE.acquire_js_reference( $0 ); return ", #js_constraints, " | 0;" ),
                            reference.as_raw()
                        ) == 1
                    }
                }
            }
        },
        None => quote! {}
    };

    let subclass_of_impl: Vec< _ > = subclass_of.into_iter().map( |target| {
        let target: syn::Ident = target.into();
        quote! {
            impl #impl_generics From< #name #ty_generics > for #target #where_clause {
                #[inline]
                fn from( value: #name #ty_generics ) -> Self {
                    let reference: ::stdweb::Reference = value.into();
                    unsafe {
                        <#target as ::stdweb::ReferenceType>::from_reference_unchecked( reference )
                    }
                }
            }

            impl #impl_generics ::stdweb::unstable::TryFrom< #target > for #name #ty_generics #where_clause {
                type Error = ::stdweb::private::ConversionError;

                #[inline]
                fn try_from( value: #target ) -> Result< Self, Self::Error > {
                    use ::stdweb::unstable::TryInto;
                    let reference: ::stdweb::Reference = value.into();
                    reference.try_into()
                }
            }
        }
    }).collect();

    let expanded = quote! {
        #(#subclass_of_impl)*
        #instance_of_impl

        impl #impl_generics AsRef< ::stdweb::Reference > for #name #ty_generics #where_clause {
            #[inline]
            fn as_ref( &self ) -> &::stdweb::Reference {
                &self.0
            }
        }

        impl #impl_generics ::stdweb::ReferenceType for #name #ty_generics #where_clause {
            #[inline]
            unsafe fn from_reference_unchecked( reference: ::stdweb::Reference ) -> Self {
                #name( reference, #default_args )
            }
        }

        impl #impl_generics From< #name #ty_generics > for ::stdweb::Reference #where_clause {
            #[inline]
            fn from( value: #name #ty_generics ) -> Self {
                value.0
            }
        }

        impl #impl_generics ::stdweb::unstable::TryFrom< #name #ty_generics > for ::stdweb::Reference #where_clause {
            type Error = ::stdweb::unstable::Void;

            #[inline]
            fn try_from( value: #name #ty_generics ) -> Result< Self, Self::Error > {
                Ok( value.0 )
            }
        }

        impl #impl_generics ::stdweb::unstable::TryFrom< ::stdweb::Reference > for #name #ty_generics #where_clause {
            type Error = ::stdweb::private::ConversionError;

            #[inline]
            fn try_from( reference: ::stdweb::Reference ) -> Result< Self, Self::Error > {
                reference.downcast()
                    .ok_or_else( || ::stdweb::private::ConversionError::Custom( "reference is of a different type".into() ) )
            }
        }

        impl< '_r, #generics_params > ::stdweb::unstable::TryFrom< &'_r ::stdweb::Reference > for #name #ty_generics #where_clause {
            type Error = ::stdweb::private::ConversionError;

            #[inline]
            fn try_from( reference: &::stdweb::Reference ) -> Result< Self, Self::Error > {
                use ::stdweb::unstable::TryInto;
                reference.clone().try_into()
            }
        }

        impl #impl_generics ::stdweb::unstable::TryFrom< ::stdweb::Value > for #name #ty_generics #where_clause {
            type Error = ::stdweb::private::ConversionError;

            #[inline]
            fn try_from( value: ::stdweb::Value ) -> Result< Self, Self::Error > {
                use ::stdweb::unstable::TryInto;
                let reference: ::stdweb::Reference = value.try_into()?;
                reference.downcast()
                    .ok_or_else( || ::stdweb::private::ConversionError::Custom( "reference is of a different type".into() ) )
            }
        }

        impl< '_r, #generics_params > ::stdweb::unstable::TryFrom< &'_r ::stdweb::Value > for #name #ty_generics #where_clause {
            type Error = ::stdweb::private::ConversionError;

            #[inline]
            fn try_from( value: &::stdweb::Value ) -> Result< Self, Self::Error > {
                use ::stdweb::unstable::TryInto;
                let reference: &::stdweb::Reference =
                    value.as_reference()
                    .ok_or_else( || ::stdweb::private::ConversionError::Custom( "not a reference".into() ) )?;

                reference.try_into()
            }
        }

        impl #impl_generics ::stdweb::private::JsSerialize for #name #ty_generics #where_clause {
            #[doc(hidden)]
            #[inline]
            fn _into_js< 'a >( &'a self, arena: &'a ::stdweb::private::PreallocatedArena ) -> ::stdweb::private::SerializedValue< 'a > {
                self.0._into_js( arena )
            }

            #[doc(hidden)]
            #[inline]
            fn _memory_required( &self ) -> usize {
                ::stdweb::Reference::_memory_required( &self.0 )
            }
        }

        impl #impl_generics ::stdweb::private::JsSerializeOwned for #name #ty_generics #where_clause {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self >, arena: &'_a ::stdweb::private::PreallocatedArena ) -> ::stdweb::private::SerializedValue< '_a > {
                ::stdweb::private::JsSerialize::_into_js( value.as_ref().unwrap(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                ::stdweb::private::JsSerialize::_memory_required( self )
            }
        }

        impl< '_r, #generics_params > ::stdweb::private::JsSerializeOwned for &'_r #name #ty_generics #where_clause {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self >, arena: &'_a ::stdweb::private::PreallocatedArena ) -> ::stdweb::private::SerializedValue< '_a > {
                ::stdweb::private::JsSerialize::_into_js( value.unwrap(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                ::stdweb::private::JsSerialize::_memory_required( *self )
            }
        }
    };

    expanded.into()
}
