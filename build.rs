extern crate gl_generator;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    // EGL
    let mut file = File::create(&Path::new(&dest).join("egl_bindings.rs")).unwrap();
    Registry::new(
        Api::Egl,
        (1, 5),
        Profile::Core,
        Fallbacks::All,
        ["EGL_KHR_image_base"],
    )
    .write_bindings(GlobalGenerator, &mut file)
    .unwrap();
}
