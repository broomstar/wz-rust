extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    if !Path::new("wz/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"]).status();
    }

    let mut cfg = cc::Build::new();

    cfg.file("libwz/src/lib/aes256.c").file("libwz/src/file.c").include("libwz/src").include("libwz/src/lib");

    #[cfg(target_os = "windows")]
    cfg.file("libwz/src/byteorder.c");

    if let Some(include) = std::env::var_os("DEP_Z_INCLUDE") {
        cfg.include(include);
    }

    cfg.compile("wz");

    let bindings =
        bindgen::Builder::default().header("libwz/src/wz.h").generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
