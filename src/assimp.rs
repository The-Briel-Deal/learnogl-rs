#![allow(clippy::all)]
// I had to do this because of u128s, apparently its not really a problem anymore though?
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Imports file and panics if it can't be found/imported.
pub fn import_file(path: &str) -> &aiScene {
    unsafe {
        aiImportFile(format!("{path}\0").as_ptr() as *const i8, 0)
            .as_ref()
            .expect(&format!("File at '{path}' not found or cannot be imported. Please try again with another path."))
    }
}
