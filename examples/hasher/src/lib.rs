#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;
extern crate sha1;

use stdweb::js_export;
use sha1::Sha1;

#[js_export]
fn sha1( string: &str ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    hasher.digest().to_string()
}
