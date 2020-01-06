#[macro_use]
extern crate stdweb;
extern crate sha1;

use stdweb::js_export;
use sha1::{Digest, Sha1};

#[js_export]
fn sha1( string: &str ) -> String {
    let hash = Sha1::digest( string.as_bytes() );
    format!( "{:x}", hash )
}
