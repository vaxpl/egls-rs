use std::convert::TryInto;
use std::ffi::{CStr, CString};

/// Foreign Function Interface of the EGL unsafe bindings.
pub mod ffi;

/// Maintain the EGL Environment.
pub mod env;

/// The shared library helper.
pub mod so;

/// Re-Export all in the ffi as top level.
pub use ffi::*;

/// Error Object for EGL.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error(EGLint);

impl Error {
    /// Create an new error with eglGetError.
    pub fn new() -> Self {
        Self {
            0: unsafe { ffi::GetError() },
        }
    }

    /// Return true if error raised.
    pub fn is_error(&self) -> bool {
        self.0 != ffi::SUCCESS.try_into().unwrap()
    }

    /// Return true if no error.
    pub fn is_okay(&self) -> bool {
        self.0 == ffi::SUCCESS.try_into().unwrap()
    }

    /// Return human reable text of the error code.
    pub fn to_str(&self) -> &str {
        match self.0 as u32 {
            ffi::SUCCESS => "No error",
            ffi::NOT_INITIALIZED => "EGL not initialized or failed to initialize",
            ffi::BAD_ACCESS => "Resource inaccessible",
            ffi::BAD_ALLOC => "Cannot allocate resources",
            ffi::BAD_ATTRIBUTE => "Unrecognized attribute or attribute value",
            ffi::BAD_CONTEXT => "Invalid EGL context",
            ffi::BAD_CONFIG => "Invalid EGL frame buffer configuration",
            ffi::BAD_CURRENT_SURFACE => "Current surface is no longer valid",
            ffi::BAD_DISPLAY => "Invalid EGL display",
            ffi::BAD_SURFACE => "Invalid surface",
            ffi::BAD_MATCH => "Inconsistent arguments",
            ffi::BAD_PARAMETER => "Invalid argument",
            ffi::BAD_NATIVE_PIXMAP => "Invalid native pixmap",
            ffi::BAD_NATIVE_WINDOW => "Invalid native window",
            ffi::CONTEXT_LOST => "Context lost",
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

/// Set the current rendering API.
///
/// # Parameters
///
/// * `api` - Specifies the client API to bind,
///           one of OPENGL_API, OPENGL_ES_API, or OPENVG_API.
pub fn bind_api(api: EGLenum) -> Result<bool, Error> {
    match unsafe { ffi::BindAPI(api) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Defines a two-dimensional texture image.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `surface` - Specifies the EGL surface.
/// * `buffer` - Specifies the texture image data.
pub fn bind_tex_image(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> Result<bool, Error> {
    match unsafe { ffi::BindTexImage(dpy, surface, buffer) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Return a list of EGL frame buffer configurations that match specified attributes.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `attrib_list` - Specifies attributes required to match by configs.
/// * `configs` - Returns an array of frame buffer configurations.
/// * `config_size` - Specifies the size of the array of frame buffer configurations.
/// * `num_config` - Returns the number of frame buffer configurations returned.
pub fn choose_config(
    display: EGLDisplay,
    attrib_list: *const EGLint,
    configs: *mut EGLConfig,
    config_size: EGLint,
    num_config: *mut EGLint,
) -> Result<bool, Error> {
    match unsafe { ffi::ChooseConfig(display, attrib_list, configs, config_size, num_config) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Create a new EGL rendering context.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `config` - Specifies the EGL frame buffer configuration that defines the frame buffer resource available to the rendering context.
/// * `share_context` - Specifies another EGL rendering context with which to share data, as defined by the client API corresponding to the contexts.
///    Data is also shared with all other contexts with which share_context shares data.
///    EGL_NO_CONTEXT indicates that no sharing is to take place.
/// * `attrib_list` - Specifies attributes and attribute values for the context being created.
///    Only the attribute EGL_CONTEXT_CLIENT_VERSION may be specified.
pub fn create_context(
    display: EGLDisplay,
    config: EGLConfig,
    share_context: EGLContext,
    attrib_list: *const EGLint,
) -> Result<EGLContext, Error> {
    let ctx = unsafe { ffi::CreateContext(display, config, share_context, attrib_list) };
    if ctx == NO_CONTEXT {
        Err(Error::new())
    } else {
        Ok(ctx)
    }
}

/// Create a new EGLImage object.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `context` - Specifies the client API context for which the image is created.
/// * `target` - Specifies the type of resource used as the image source.
/// * `buffer` - Specifies the resource to be used as the image source.
/// * `attrib_list` - Specifies attributes used to select sub-sections of the resource to be used as the image source.
pub fn create_image(
    display: EGLDisplay,
    context: EGLContext,
    target: EGLenum,
    buffer: EGLClientBuffer,
    attrib_list: *const EGLAttrib,
) -> Result<EGLImage, Error> {
    let img = unsafe { ffi::CreateImage(display, context, target, buffer, attrib_list) };
    if img == NO_IMAGE {
        Err(Error::new())
    } else {
        Ok(img)
    }
}

/// Create a new EGLImage object (KHR Ext).
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `context` - Specifies the client API context for which the image is created.
/// * `target` - Specifies the type of resource used as the image source.
/// * `buffer` - Specifies the resource to be used as the image source.
/// * `attrib_list` - Specifies attributes used to select sub-sections of the resource to be used as the image source.
pub fn create_image_khr(
    display: EGLDisplay,
    context: EGLContext,
    target: EGLenum,
    buffer: EGLClientBuffer,
    attrib_list: *const EGLint,
) -> Result<EGLImage, Error> {
    let val = unsafe { ffi::CreateImageKHR(display, context, target, buffer, attrib_list) };
    if val == NO_IMAGE {
        Err(Error::new())
    } else {
        Ok(val)
    }
}

/// Create a new EGL pixel buffer surface.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `config` - Specifies the EGL frame buffer configuration that defines the frame buffer resource available to the surface.
/// * `attrib_list` - Specifies pixel buffer surface attributes. May be NULL or empty (first attribute is EGL_NONE).
pub fn create_pbuffer_surface(
    display: EGLDisplay,
    config: EGLConfig,
    attrib_list: *const EGLint,
) -> Result<EGLSurface, Error> {
    let val = unsafe { ffi::CreatePbufferSurface(display, config, attrib_list) };
    if val == NO_SURFACE {
        Err(Error::new())
    } else {
        Ok(val)
    }
}

/// Create a new EGL offscreen surface.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `config` - Specifies the EGL frame buffer configuration that defines the frame buffer resource available to the surface.
/// * `native_pixmap` - Specifies the native pixmap.
/// * `attrib_list` - Specifies pixmap surface attributes. May be NULL or empty (first attribute is EGL_NONE).
pub fn create_pixmap_surface(
    display: EGLDisplay,
    config: EGLConfig,
    pixmap: EGLNativePixmapType,
    attrib_list: *const EGLint,
) -> Result<EGLSurface, Error> {
    let val = unsafe { ffi::CreatePixmapSurface(display, config, pixmap, attrib_list) };
    if val == NO_SURFACE {
        Err(Error::new())
    } else {
        Ok(val)
    }
}

/// Create a new EGL window surface.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `config` - Specifies the EGL frame buffer configuration that defines the frame buffer resource available to the surface.
/// * `native_window` - Specifies the native window.
/// * `attrib_list` - Specifies window surface attributes. May be NULL or empty (first attribute is EGL_NONE).
pub fn create_window_surface(
    display: EGLDisplay,
    config: types::EGLConfig,
    win: EGLNativeWindowType,
    attrib_list: *const EGLint,
) -> Result<EGLSurface, Error> {
    let val = unsafe { ffi::CreateWindowSurface(display, config, win, attrib_list) };
    if val == NO_SURFACE {
        Err(Error::new())
    } else {
        Ok(val)
    }
}

/// Destroy an EGL rendering context.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `context` - Specifies the EGL rendering context to be destroyed.
pub fn destroy_context(display: EGLDisplay, ctx: EGLContext) -> Result<bool, Error> {
    match unsafe { ffi::DestroyContext(display, ctx) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Destroy an EGLImage object.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `image` - Specifies the image to destroy.
pub fn destroy_image(display: EGLDisplay, image: EGLImage) -> Result<bool, Error> {
    match unsafe { ffi::DestroyImage(display, image) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Destroy an EGLImage object (KHR Ext).
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `image` - Specifies the image to destroy.
pub fn destroy_image_khr(display: EGLDisplay, image: EGLImageKHR) -> Result<bool, Error> {
    match unsafe { ffi::DestroyImageKHR(display, image) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Destroy an EGL surface.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `surface` - Specifies the EGL surface to be destroyed.
pub fn destroy_surface(display: EGLDisplay, surface: EGLSurface) -> Result<bool, Error> {
    match unsafe { ffi::DestroySurface(display, surface) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Return an EGL display connection.
///
/// # Parameters
///
/// * `native_display` - Specifies the display to connect to.
///                      EGL_DEFAULT_DISPLAY indicates the default display.
pub fn get_display(native_display: EGLNativeDisplayType) -> Result<EGLDisplay, Error> {
    let ptr = unsafe { ffi::GetDisplay(native_display) };
    if ptr.is_null() {
        Err(Error::new())
    } else {
        Ok(ptr)
    }
}

/// Return the default EGL display connection.
#[inline]
pub fn get_default_display() -> Result<EGLDisplay, Error> {
    get_display(DEFAULT_DISPLAY)
}

/// Initialize an EGL display connection.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection to initialize.
/// * `major` - Returns the major version number of the EGL implementation. May be NULL.
/// * `minor` - Returns the minor version number of the EGL implementation. May be NULL.
pub fn initialize(dpy: EGLDisplay, major: &mut EGLint, minor: &mut EGLint) -> Result<bool, Error> {
    match unsafe { ffi::Initialize(dpy, major, minor) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

///  Initialize an EGL display connection with default version.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection to initialize.
pub fn initialize_default(dpy: EGLDisplay) -> Result<bool, Error> {
    match unsafe { ffi::Initialize(dpy, std::ptr::null_mut(), std::ptr::null_mut()) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Attach an EGL rendering context to EGL surfaces.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `draw` - Specifies the EGL draw surface.
/// * `read` - Specifies the EGL read surface.
/// * `context` - Specifies the EGL rendering context to be attached to the surfaces.
pub fn make_current(
    display: EGLDisplay,
    draw: EGLSurface,
    read: EGLSurface,
    ctx: EGLContext,
) -> Result<bool, Error> {
    match unsafe { ffi::MakeCurrent(display, draw, read, ctx) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Query the current rendering API.
///
/// eglQueryAPI returns the value of the current rendering API for EGL in the thread it is called from.
/// The current rendering API is set by eglBindAPI, and affects the behavior of other EGL commands.
/// The value returned will be one of the valid api parameters to eglBindAPI, or EGL_NONE.
pub fn query_api() -> EGLenum {
    unsafe { ffi::QueryAPI() }
}

/// Return EGL rendering context information.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `context` - Specifies the EGL rendering context to query.
/// * `attribute` - Specifies the EGL rendering context attribute to be returned.
pub fn query_context(dpy: EGLDisplay, ctx: EGLContext, attr: EGLint) -> Result<EGLint, Error> {
    let mut value: EGLint = 0;
    match unsafe { ffi::QueryContext(dpy, ctx, attr, &mut value) } {
        1 => Ok(value),
        _ => Err(Error::new()),
    }
}

/// Return a string describing properties of the EGL client or of an EGL display connection.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `name` - Specifies a symbolic constant, one of EGL_CLIENT_APIS, EGL_VENDOR, EGL_VERSION, or EGL_EXTENSIONS.
pub fn query_string(dpy: EGLDisplay, name: EGLint) -> Result<String, Error> {
    unsafe {
        let ptr = ffi::QueryString(dpy, name);
        if ptr.is_null() {
            Err(Error::new())
        } else {
            Ok(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }
}

/// Releases a color buffer that is being used as a texture.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `surface` - Specifies the EGL surface.
/// * `buffer` - Specifies the texture image data.
pub fn release_tex_image(
    display: types::EGLDisplay,
    surface: types::EGLSurface,
    buffer: types::EGLint,
) -> Result<bool, Error> {
    match unsafe { ffi::ReleaseTexImage(display, surface, buffer) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Post EGL surface color buffer to a native window.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `surface` - Specifies the EGL drawing surface whose buffers are to be swapped.
pub fn swap_buffers(display: EGLDisplay, surface: EGLSurface) -> Result<bool, Error> {
    match unsafe { ffi::SwapBuffers(display, surface) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Specifies the minimum number of video frame periods per buffer swap for the window associated with the current context.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection.
/// * `interval` - Specifies the minimum number of video frames that are displayed before a buffer swap will occur.
pub fn swap_interval(display: EGLDisplay, interval: EGLint) -> Result<bool, Error> {
    match unsafe { ffi::SwapInterval(display, interval) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Terminate an EGL display connection.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection to terminate.
pub fn terminate(display: types::EGLDisplay) -> Result<bool, Error> {
    match unsafe { ffi::Terminate(display) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::so::SharedObject;
    use super::*;
    use std::convert::TryInto;

    fn egl_init() -> (SharedObject, EGLDisplay) {
        let so = SharedObject::load("libEGL.so");
        ffi::load_with(|s| so.get_proc_address(s));
        ffi::load_with_priv(|s| so.get_proc_address(s));
        let dpy = get_default_display().unwrap();
        let mut major: EGLint = 3;
        let mut minor: EGLint = 2;
        assert!(initialize(dpy, &mut major, &mut minor).unwrap());
        (so, dpy)
    }

    #[test]
    fn test_bind_api() {
        let (_so, _dpy) = egl_init();
        assert!(bind_api(ffi::OPENGL_ES_API).unwrap());
    }

    #[test]
    fn test_query_string() {
        let (_so, dpy) = egl_init();
        let apis = query_string(dpy, ffi::CLIENT_APIS as EGLint).unwrap();
        assert_eq!(apis, "OpenGL_ES");
    }
}
