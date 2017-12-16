#[macro_use]
extern crate stdweb;
extern crate sha1;

use sha1::Sha1;

fn hash( string: String ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    hasher.digest().to_string()
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.sha1 = @{hash};
    }
}
