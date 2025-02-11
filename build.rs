use std::env;
use std::path::PathBuf;

fn main() {
    // let flutter_libraries_path = std::env::var("LD_LIBRARY_PATH")
    //     .expect("fill LD_LIBRARY_PATH env var to libflutter_engine.so directory");

    // println!("cargo:rustc-link-search=native={}", flutter_libraries_path);
    // Tell cargo to tell rustc to link the flutter_engine
    // shared library.
    println!("cargo:rustc-link-lib=flutter_engine");
    // println!(
    //     "cargo:rustc-link-search=native={}/linux",
    //     env::var("CARGO_MANIFEST_DIR").unwrap()
    // );

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=flutter_embedder.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("flutter_embedder.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("embedder.rs"))
        .expect("Couldn't write bindings!");
}
