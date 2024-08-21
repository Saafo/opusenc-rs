fn main() {
    println!("cargo:rustc-link-lib=opusenc");
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(feature = "bindgen")]
    bindgen()
}

#[cfg(feature = "bindgen")]
fn bindgen() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(["-isystem", "/usr/include/opus"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .generate()
        .expect("Unable to generate bindings");

    let pwd = std::env::current_dir().unwrap();
    bindings
        .write_to_file(pwd.join("src").join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
