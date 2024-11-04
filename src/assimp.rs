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
        let scene = aiImportFile(
            format!("{path}\0").as_ptr() as *const i8,
            aiPostProcessSteps_aiProcess_Triangulate | aiPostProcessSteps_aiProcess_FlipUVs,
        )
        .as_ref()
        .expect(&format!(
            "File at '{path}' not found or cannot be imported. Please try again with another path."
        ));
        assert_eq!(scene.mFlags & AI_SCENE_FLAGS_INCOMPLETE, 0);
        assert!(!scene.mRootNode.is_null());
        scene
    }
}
