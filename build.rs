use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=bz2");

    println!("cargo:rustc-link-search=extern-sdcc");
    bindgen::Builder::default()
        .header("extern-sdcc/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from("src/ccompilers").join("sdcc.rs"))
        .expect("Couldn't write bindings!");
}
