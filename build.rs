extern crate rustc_version;
use rustc_version::{version, version_meta, Channel};

fn main() {
    let mut current = version().unwrap();
    current.pre = Vec::new();

    if version_meta().unwrap().channel == Channel::Nightly {
        println!( "cargo:rustc-cfg=rust_nightly" );
    }
}
