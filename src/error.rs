use crate::{egl, EGLint};
use std::convert::TryInto;

/// Error Object for EGL.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Error(EGLint);

impl Error {
    /// Create an new error with eglGetError.
    pub fn new() -> Self {
        Self {
            0: unsafe { egl::GetError() },
        }
    }

    /// Return true if error raised.
    pub fn is_error(self) -> bool {
        self.0 != egl::SUCCESS.try_into().unwrap()
    }

    /// Return true if no error.
    pub fn is_okay(self) -> bool {
        self.0 == egl::SUCCESS.try_into().unwrap()
    }

    /// Return human reable text of the error code.
    pub fn to_str(self) -> &'static str {
        match self.0 as u32 {
            egl::SUCCESS => "No error",
            egl::NOT_INITIALIZED => "EGL not initialized or failed to initialize",
            egl::BAD_ACCESS => "Resource inaccessible",
            egl::BAD_ALLOC => "Cannot allocate resources",
            egl::BAD_ATTRIBUTE => "Unrecognized attribute or attribute value",
            egl::BAD_CONTEXT => "Invalid EGL context",
            egl::BAD_CONFIG => "Invalid EGL frame buffer configuration",
            egl::BAD_CURRENT_SURFACE => "Current surface is no longer valid",
            egl::BAD_DISPLAY => "Invalid EGL display",
            egl::BAD_SURFACE => "Invalid surface",
            egl::BAD_MATCH => "Inconsistent arguments",
            egl::BAD_PARAMETER => "Invalid argument",
            egl::BAD_NATIVE_PIXMAP => "Invalid native pixmap",
            egl::BAD_NATIVE_WINDOW => "Invalid native window",
            egl::CONTEXT_LOST => "Context lost",
            _ => "Unknown error",
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error({}, \"{}\")", self.0, self.to_str())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error({}, \"{}\")", self.0, self.to_str())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.to_str()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
