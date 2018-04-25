extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if !Path::new("libwz/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let bindings = bindgen::Builder::default()
        .header("libwz/src/wz.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = cmake::Config::new("libwz").cflag("-Wno-comma").build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=wz");
}
