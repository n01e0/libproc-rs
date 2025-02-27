#[cfg(target_os = "macos")]
fn main() {
    use std::env;
    use std::path::Path;

    let bindings = bindgen::builder()
        .header_contents("libproc_rs.h", "#include <libproc.h>")
        .layout_tests(false)
        .generate()
        .expect("Failed to build libproc bindings");

    let output_path = Path::new(&env::var("OUT_DIR").expect("OUT_DIR env var was not defined"))
        .join("osx_libproc_bindings.rs");

    bindings
        .write_to_file(output_path)
        .expect("Failed to write libproc bindings");
}

#[cfg(not(target_os = "macos"))]
fn main() {}
