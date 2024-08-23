use std::env;
use std::path::PathBuf;

fn main() {
    println!(
        "cargo:rustc-link-search={}",
        env::var("SHARED_LIBRARY_DIR").unwrap()
    );

    println!("cargo:rustc-link-lib=hdfs");

    let bindings = bindgen::Builder::default()
        .header("headers.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
