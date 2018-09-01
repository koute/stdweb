use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn read< P: AsRef< Path > >( path: P ) -> Result< String, io::Error > {
    let mut fp = File::open( path.as_ref() )?;
    let mut output = String::new();
    fp.read_to_string( &mut output )?;
    Ok( output )
}

fn generate_body( output: &mut String ) -> &'static str {
    output.push_str( "$sub!( r##\"" );

    let target = env::var( "TARGET" ).expect( "no TARGET defined" );
    let target_specific_runtime_path = match target.as_str() {
        "wasm32-unknown-unknown" =>
            "src/runtime_wasm.js",
        _ =>
            "src/runtime_emscripten.js"
    };

    let runtime = read( "src/runtime.js" ).expect( "cannot read runtime" );
    let target_specific_runtime = read( target_specific_runtime_path ).expect( "cannot read target specific runtime" );
    for line in runtime.lines().chain( target_specific_runtime.lines() ) {
        if let Some( position ) = line.find( "//" ) { // Strip out comments.
            output.push_str( &line[ 0..position ] );
        } else {
            output.push_str( &line );
        }
        output.push_str( " " );
    }

    output.push_str( "\"## )" );
    target_specific_runtime_path
}

fn main() {
    let out_dir = env::var( "OUT_DIR" ).expect( "no OUT_DIR defined" );
    let out_path = Path::new( &out_dir ).join( "runtime.rs" );
    let mut fp = File::create( &out_path ).expect( "cannot create a file in OUT_DIR" );

    let separator = if cfg!( windows ) { "\\" } else { "/" };
    println!( "cargo:rustc-env=PATH_SEPARATOR={}", separator );

    let mut output = String::new();
    output.push_str( "#[doc(hidden)]" );
    output.push_str( "#[macro_export]" );
    output.push_str( "macro_rules! stdweb_internal_runtime_initialize { ($sub:tt) => {" );

    // We need to emit the runtime ourselves if we're not being compiled under cargo-web.
    if !env::var( "COMPILING_UNDER_CARGO_WEB" ).map( |var| var == "1" ).unwrap_or( false ) {
        let target_specific_runtime_path = generate_body( &mut output );
        println!( "cargo:rerun-if-changed=src/runtime.js" );
        println!( "cargo:rerun-if-changed={}", target_specific_runtime_path );
    }

    output.push_str( "}}" );

    fp.write_all( output.as_bytes() ).unwrap();
    println!( "cargo:rerun-if-env-changed=COMPILING_UNDER_CARGO_WEB" );
}
