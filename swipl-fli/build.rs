use std::env;
use std::path::PathBuf;

use swipl_info::*;

fn main() {
    let info = get_swipl_info();
    println!("cargo:rustc-link-lib={}", info.lib_name);
    println!("cargo:rustc-link-search={}", info.lib_dir);
    println!("cargo:rerun-if-changed=c/wrapper.h");
    println!("cargo:rerun-if-env-changed=SWIPL");

    // Autodetect SWI-Prolog 10.x (version >= 100000)
    // SWI-Prolog 10.x uses C11 bool for FLI return types
    if info.version >= 100000 {
        println!("cargo:rustc-cfg=swipl10");
    }

    let bindings = bindgen::Builder::default()
        .header("c/wrapper.h")
        .clang_arg(format!("-I{}", info.header_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
