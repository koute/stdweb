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

fn main() {
    let out_dir = env::var( "OUT_DIR" ).expect( "no OUT_DIR defined" );
    let out_path = Path::new( &out_dir ).join( "runtime.rs" );
    let mut fp = File::create( &out_path ).expect( "cannot create a file in OUT_DIR" );

    let mut output = String::new();
    output.push_str( "r##\"" );

    let target = env::var( "TARGET" ).expect( "no TARGET defined" );
    let target_specific_runtime_path = match target.as_str() {
        "wasm32-unknown-unknown" =>
            "src/webcore/runtime_wasm.js",
        _ =>
            "src/webcore/runtime_emscripten.js"
    };

    let runtime = read( "src/webcore/runtime.js" ).expect( "cannot read runtime" );
    let target_specific_runtime = read( target_specific_runtime_path ).expect( "cannot read target specific runtime" );
    for line in runtime.lines().chain( target_specific_runtime.lines() ) {
        if let Some( position ) = line.find( "//" ) { // Strip out comments.
            output.push_str( &line[ 0..position ] );
        } else {
            output.push_str( &line );
        }
        output.push_str( " " );
    }

    output.push_str( "\"##" );

    fp.write_all( output.as_bytes() ).unwrap();
    println!( "cargo:rerun-if-changed=src/webcore/runtime.js" );
    println!( "cargo:rerun-if-changed={}", target_specific_runtime_path );
}
