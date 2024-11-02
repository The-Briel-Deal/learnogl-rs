use std::{env, fs::File, path::PathBuf};

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

fn main() {
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    // Assimp
    println!("cargo:rustc-link-lib=assimp");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_var("FP_NAN")
        .blocklist_var("FP_SUBNORMAL")
        .blocklist_var("FP_NORMAL")
        .blocklist_var("FP_INFINITE")
        .blocklist_var("FP_ZERO")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(dest.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Gl Bindgen
    let mut file = File::create(dest.join("gl_bindings.rs")).unwrap();
    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .unwrap();
}
