use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=bz2");

    println!("cargo:rustc-link-search=assembler");
    bindgen::Builder::default()
        .header("assembler/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from("src/assembler").join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=ccompiler");
    bindgen::Builder::default()
        .header("ccompiler/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from("src/ccompiler").join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
