use syn;
use proc_macro2::Span;

#[derive(Copy, Clone)]
pub enum Target {
    Emscripten,
    NativeWebAssembly
}

pub fn dummy_idents( count: usize ) -> impl Iterator< Item = syn::Ident > {
    (0..count).into_iter().map( |nth| {
        syn::Ident::new( &format!( "a{}", nth ), Span::call_site() )
    })
}
