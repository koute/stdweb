use std::fmt::Write;
use proc_macro2::{TokenTree, Delimiter};
use syn;
use syn::buffer::Cursor;
use syn::parse::{Result, ParseStream};
use quote::ToTokens;

enum Chunk {
    Text( String ),
    Block( syn::Block )
}

pub struct StringifiedCode {
    chunks: Vec< Chunk >
}

fn can_trim_whitespace_next_to( ch: char ) -> bool {
    ch.is_whitespace() || "=+-*/%<>&^|~?:{}()[];.,".contains( ch )
}

impl StringifiedCode {
    fn push( &mut self, string: &str ) {
        match self.chunks.last_mut() {
            None | Some( Chunk::Block( .. ) ) => {},
            Some( Chunk::Text( ref mut buffer ) ) => {
                let l = buffer.chars().last().unwrap_or( ' ' );
                let r = string.chars().next().unwrap_or( ' ' );
                if !can_trim_whitespace_next_to( l ) && !can_trim_whitespace_next_to( r ) {
                    buffer.push( ' ' );
                }
                buffer.push_str( string );
                return;
            }
        }

        self.chunks.push( Chunk::Text( string.into() ) );
    }

    fn push_block( &mut self, block: syn::Block ) {
        self.chunks.push( Chunk::Block( block ) );
    }

    pub fn arg_count( &self ) -> usize {
        self.chunks.iter().filter( |chunk|
            match chunk {
                Chunk::Block( .. ) => true,
                _ => false
            }
        ).count()
    }

    pub fn code( &self, initial_placeholder_index: usize ) -> String {
        let capacity = self.chunks.iter().map( |chunk|
            match chunk {
                Chunk::Text( text ) => text.len(),
                Chunk::Block( .. ) => 4
            }
        ).fold( 0, |sum, len| sum + len );

        let mut counter = initial_placeholder_index;
        let mut output = String::with_capacity( capacity );
        for chunk in &self.chunks {
            match chunk {
                Chunk::Text( text ) => output.push_str( text ),
                Chunk::Block( _ ) => {
                    write!( output, "(${})", counter ).unwrap();
                    counter += 1;
                }
            }
        }

        output
    }
}

fn stringify< 'a >( mut cursor: Cursor< 'a >, output: &mut StringifiedCode ) -> Result< Cursor< 'a > > {
    while let Some( (tt, next) ) = cursor.token_tree() {
        cursor = match tt {
            TokenTree::Punct( ref punct ) if punct.as_char() == '@' && next.group( Delimiter::Brace ).is_some() => {
                let (tt, next_next) = next.token_tree().unwrap();
                output.push_block( syn::parse2( tt.into_token_stream() )? );
                next_next
            },
            TokenTree::Group( ref group ) => {
                let (start, end) = match group.delimiter() {
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::None => ("", "")
                };

                output.push( start );
                let inner = cursor.group( group.delimiter() ).unwrap().0;
                stringify( inner, output )?;
                output.push( end );
                next
            },
            _ => {
                let token = tt.to_string();
                output.push( &token );
                next
            }
        };
    }

    Ok( cursor )
}

impl syn::parse::Parse for StringifiedCode {
    fn parse( input: ParseStream ) -> Result< Self > {
        input.step( |cursor| {
            let mut output = StringifiedCode {
                chunks: Vec::new()
            };
            let cursor = stringify( *cursor, &mut output )?;
            Ok( (output, cursor) )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::StringifiedCode;
    use proc_macro2::TokenStream;

    fn assert_stringify( input: TokenStream, initial_placeholder: usize, expected: &str ) {
        let snippet: StringifiedCode = syn::parse2( input ).unwrap();
        assert_eq!( snippet.code( initial_placeholder ), expected );
    }

    #[test]
    fn test_stringify() {
        assert_stringify( quote! { return thing; }, 0, "return thing;" );
        assert_stringify( quote! { console.log }, 0, "console.log" );
        assert_stringify( quote! { 1.0 }, 0, "1.0" );
        assert_stringify( quote! { [ 1.0 ] }, 0, "[1.0]" );
        assert_stringify( quote! { { 1.0 } }, 0, "{1.0}" );
        assert_stringify( quote! { ( 1.0 ) }, 0, "(1.0)" );
        assert_stringify( quote! { a b }, 0, "a b" );
        assert_stringify( quote! { === }, 0, "===" );
        assert_stringify( quote! { ++i }, 0, "++i" );
        assert_stringify( quote! { i++ }, 0, "i++" );
        assert_stringify( quote! { --i }, 0, "--i" );
        assert_stringify( quote! { i-- }, 0, "i--" );
        assert_stringify( quote! { return _.sum([1, 2]); }, 0, "return _.sum([1,2]);" );
        assert_stringify( quote! { return $; }, 0, "return $;" );
        assert_stringify( quote! { ( @{1} ); }, 0, "(($0));" );
        assert_stringify(
            quote! { console.log( "Hello!", @{1234i32} ); },
            0,
            "console.log(\"Hello!\",($0));"
        );
        assert_stringify(
            quote! { @{a}.fn( @{b} ); },
            0,
            "($0).fn(($1));"
        );
        assert_stringify(
            quote! { @{a}.fn( @{b} ); },
            1,
            "($1).fn(($2));"
        );
    }
}
