use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rerun-if-changed=lib/lib.h");
    println!("cargo:rerun-if-changed=lib/lib.c");
    println!("cargo:rerun-if-changed=lib/CMakeLists.txt");

    // Build the C library using CMake
    let dst = cmake::Config::new("lib").build_target("ffi_lib").build();

    // Find the built library
    let lib_path = dst.join("build");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=ffi_lib");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("lib/lib.h")
        .allowlist_type("Packet")
        .allowlist_function("get_packet_len")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
