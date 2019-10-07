use syn::parse::{ParseStream, Parse, Result};

// Since currently producing an expression from a procedural
// macro is not stable we use something like this to work around
// this problem.
pub struct AttrHack< T: Parse > {
    pub fn_name: syn::Ident,
    pub return_ty: Option< syn::Type >,
    pub inner: T
}

impl< T > Parse for AttrHack< T > where T: Parse {
    fn parse( input: ParseStream ) -> Result< Self > {
        input.parse::< Token![fn] >()?;
        let fn_name = input.parse::< syn::Ident >()?;

        #[allow(unused_variables)]
        let fn_args_input;
        parenthesized!( fn_args_input in input );

        let return_ty =
            if input.peek( Token![->] ) {
                let _: Token![->] = input.parse()?;
                Some( input.parse()? )
            } else {
                None
            };

        let fn_body_input;
        braced!( fn_body_input in input );

        let ident = fn_body_input.parse::< syn::Ident >()?;
        if ident == "call" {
            fn_body_input.parse::< Token![!] >()?;

            let inner;
            parenthesized!( inner in fn_body_input );
            let inner = inner.parse::< T >()?;
            fn_body_input.parse::< Token![;] >()?;

            Ok( AttrHack {
                fn_name,
                return_ty,
                inner
            })
        } else {
            let ident_str = ident.to_string();
            Err( syn::Error::new_spanned( ident, format!( "unexpected ident '{}'", ident_str ) ) )
        }
    }
}
