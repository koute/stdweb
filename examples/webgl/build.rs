extern crate webgl_generator;

use webgl_generator::*;
use std::env;
use std::fs::File;
use std::path::*;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("webgl_rendering_context.rs")).unwrap();

    Registry::new(Api::WebGl2, Exts::NONE)
        .write_bindings(StdwebGenerator, &mut file)
        .unwrap();
}
