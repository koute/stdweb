use std::fmt;

use stdweb::Once;
use stdweb::unstable::TryInto;

pub struct Stderr;
impl fmt::Write for Stderr {
    fn write_str( &mut self, str: &str ) -> fmt::Result {
        js! {
            process.stderr.write( @{str} );
        }

        Ok(())
    }
}

// Since `println!` doesn't work right now on `wasm32-unknown-unknown` we define our own.
macro_rules! eprintln {
    ($($token:tt)*) => {{
        #[allow(unused_imports)]
        use std::fmt::Write;
        writeln!( ::utils::Stderr, $($token)* ).unwrap()
    }}
}

// `std::process::exit` also doesn't work.
pub fn exit( status: u32 ) {
    js! {
        process.exit( @{status} );
    }
}

pub fn test< F: FnOnce() + 'static >( name: &str, callback: F ) {
    eprintln!( "Running test '{}'...", name );
    let result = js!(
        try {
            @{Once( callback )}();
        } catch( exception ) {
            process.stderr.write( exception + "\n" );
            return false;
        }

        return true;
    );

    let result: bool = result.try_into().unwrap();
    if !result {
        exit( 1 );
    }
}
