use proc_macro2;
use quote::ToTokens;

#[cfg(test)]
pub fn rust_pretty_print( code: &proc_macro2::TokenStream ) -> String {
    use std::process::{Command, Stdio};
    use std::mem;
    use std::io::{Read, Write};

    let mut cmd = Command::new( "rustfmt" );
    cmd.arg( "--version" );
    let has_rustfmt = cmd.output().map( |output| output.status.success() ).unwrap_or( false );
    if !has_rustfmt {
        return format!( "{}", code );
    }

    let mut cmd = Command::new( "rustfmt" );
    cmd.stdin( Stdio::piped() );
    cmd.stdout( Stdio::piped() );
    cmd.stderr( Stdio::null() );
    let mut child = cmd.spawn().expect( "cannot spawn rustfmt" );
    let mut stdin = child.stdin.take().unwrap();
    write!( stdin, "{}", code ).unwrap();
    mem::drop( stdin );

    let mut pretty_code = String::new();
    let mut stdout = child.stdout.take().unwrap();
    stdout.read_to_string( &mut pretty_code ).unwrap();
    child.wait().expect( "rustfmt failed" );

    pretty_code
}

#[cfg(test)]
pub fn assert_code_eq< T: ToTokens, U: ToTokens >( actual: T, expected: U ) {
    let actual = actual.into_token_stream();
    let expected = expected.into_token_stream();
    if format!( "{}", actual ) != format!( "{}", expected ) {
        let expected_pretty = rust_pretty_print( &expected );
        let actual_pretty = rust_pretty_print( &actual );
        if expected_pretty != actual_pretty {
            println!( "Expected:\n{}", expected_pretty );
            println!( "Actual:\n{}", actual_pretty );
        } else {
            println!( "Expected:\n{}", expected );
            println!( "Actual:\n{}", actual );
        }
        panic!( "Expected different generated code!" );
    }
}
