extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=rustlearningcpp");
    println!("cargo:rustc-link-search=dependency=../build/cpp/");
    println!("cargo:rerun-if-changed=../cpp/include/cpplib.h");

    let bindings = bindgen::Builder::default()
        .header("../cpp/include/cpplib.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
