extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    let mut current = version().unwrap();
    current.pre = Vec::new();

    if current >= Version::parse( "1.30.0" ).unwrap() {
        println!( "cargo:rustc-cfg=rust_1_30_or_newer" );
    }
}
