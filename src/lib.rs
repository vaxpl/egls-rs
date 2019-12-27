/// Foreign Function Interface of the EGL unsafe bindings.
pub mod egl;
pub use egl::{
    load_with, EGLAttrib, EGLClientBuffer, EGLConfig, EGLContext, EGLDisplay, EGLImage,
    EGLImageKHR, EGLNativeDisplayType, EGLNativePixmapType, EGLNativeWindowType, EGLSurface,
    EGLenum, EGLint,
};

pub mod apis;
pub use apis::*;

/// Maintain the EGL Environment.
pub mod env;
pub use env::*;

pub mod error;
pub use error::*;

pub mod image;
pub use image::{Image, ImageBuffer, ImageBuilder, Target};

pub mod pixmap;
pub use pixmap::{Pixmap, PixmapBuilder, PixmapFormat};

/// The shared library helper.
pub mod so;
