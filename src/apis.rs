use crate::{
    egl, EGLAttrib, EGLClientBuffer, EGLConfig, EGLContext, EGLDisplay, EGLImage, EGLImageKHR,
    EGLNativeDisplayType, EGLNativePixmapType, EGLNativeWindowType, EGLSurface, EGLenum, EGLint,
    Error,
};
use std::ffi::{CStr, CString};

/// Set the current rendering API.
///
/// # Parameters
///
/// * `api` - Specifies the client API to bind,
///           one of OPENGL_API, OPENGL_ES_API, or OPENVG_API.
pub fn bind_api(api: EGLenum) -> Result<bool, Error> {
    match unsafe { egl::BindAPI(api) } {
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
    match unsafe { egl::BindTexImage(dpy, surface, buffer) } {
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
///
/// # Returns
///
/// * An array of frame buffer configurations.
pub fn choose_config(display: EGLDisplay, attrib_list: &[EGLint]) -> Result<Vec<EGLConfig>, Error> {
    unsafe {
        let mut num_config: EGLint = 0;
        if egl::TRUE
            != egl::ChooseConfig(
                display,
                attrib_list.as_ptr(),
                std::ptr::null_mut(),
                0,
                &mut num_config,
            )
        {
            return Err(Error::new());
        }
        let mut configs: Vec<EGLConfig> = vec![std::ptr::null_mut(); num_config as usize];
        if egl::TRUE
            != egl::ChooseConfig(
                display,
                attrib_list.as_ptr(),
                configs.as_mut_ptr(),
                configs.len() as EGLint,
                &mut num_config,
            )
        {
            return Err(Error::new());
        }
        Ok(configs)
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
    attrib_list: Option<&[EGLint]>,
) -> Result<EGLContext, Error> {
    let aptr = attrib_list.map_or(std::ptr::null(), |v| v.as_ptr());
    let ctx = unsafe { egl::CreateContext(display, config, share_context, aptr) };
    if ctx == egl::NO_CONTEXT {
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
    attrib_list: Option<&[EGLAttrib]>,
) -> Result<EGLImage, Error> {
    let aptr = attrib_list.map_or(std::ptr::null(), |v| v.as_ptr());
    let img = unsafe { egl::CreateImage(display, context, target, buffer, aptr) };
    if img == egl::NO_IMAGE {
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
    attrib_list: Option<&[EGLint]>,
) -> Result<EGLImage, Error> {
    let aptr = attrib_list.map_or(std::ptr::null(), |v| v.as_ptr());
    let val = unsafe { egl::CreateImageKHR(display, context, target, buffer, aptr) };
    if val == egl::NO_IMAGE {
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
    attrib_list: Option<&[EGLint]>,
) -> Result<EGLSurface, Error> {
    let aptr = attrib_list.map_or(std::ptr::null(), |v| v.as_ptr());
    let val = unsafe { egl::CreatePbufferSurface(display, config, aptr) };
    if val == egl::NO_SURFACE {
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
    attrib_list: Option<&[EGLint]>,
) -> Result<EGLSurface, Error> {
    let aptr = attrib_list.map_or(std::ptr::null(), |v| v.as_ptr());
    let val = unsafe { egl::CreatePixmapSurface(display, config, pixmap, aptr) };
    if val == egl::NO_SURFACE {
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
    config: EGLConfig,
    win: EGLNativeWindowType,
    attrib_list: Option<&[EGLint]>,
) -> Result<EGLSurface, Error> {
    let aptr = attrib_list.map_or(std::ptr::null(), |v| v.as_ptr());
    let val = unsafe { egl::CreateWindowSurface(display, config, win, aptr) };
    if val == egl::NO_SURFACE {
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
    match unsafe { egl::DestroyContext(display, ctx) } {
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
    match unsafe { egl::DestroyImage(display, image) } {
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
    match unsafe { egl::DestroyImageKHR(display, image) } {
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
    match unsafe { egl::DestroySurface(display, surface) } {
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
    let ptr = unsafe { egl::GetDisplay(native_display) };
    if ptr.is_null() {
        Err(Error::new())
    } else {
        Ok(ptr)
    }
}

/// Return the default EGL display connection.
#[inline]
pub fn get_default_display() -> Result<EGLDisplay, Error> {
    get_display(egl::DEFAULT_DISPLAY)
}

/// Return a GL or an EGL extension function.
///
/// # Parameters
///
/// * `procname` - Specifies the name of the function to return.
pub fn get_proc_address<T: AsRef<str>>(procname: T) -> *mut std::os::raw::c_void {
    let procname = CString::new(procname.as_ref()).unwrap();
    unsafe { egl::GetProcAddress(procname.as_ptr()) as *mut std::os::raw::c_void }
}

/// Initialize an EGL display connection.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection to initialize.
/// * `major` - Returns the major version number of the EGL implementation. May be NULL.
/// * `minor` - Returns the minor version number of the EGL implementation. May be NULL.
pub fn initialize(dpy: EGLDisplay, major: &mut EGLint, minor: &mut EGLint) -> Result<bool, Error> {
    match unsafe { egl::Initialize(dpy, major, minor) } {
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
    match unsafe { egl::Initialize(dpy, std::ptr::null_mut(), std::ptr::null_mut()) } {
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
    match unsafe { egl::MakeCurrent(display, draw, read, ctx) } {
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
    unsafe { egl::QueryAPI() }
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
    match unsafe { egl::QueryContext(dpy, ctx, attr, &mut value) } {
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
        let ptr = egl::QueryString(dpy, name);
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
    display: EGLDisplay,
    surface: EGLSurface,
    buffer: EGLint,
) -> Result<bool, Error> {
    match unsafe { egl::ReleaseTexImage(display, surface, buffer) } {
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
    match unsafe { egl::SwapBuffers(display, surface) } {
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
    match unsafe { egl::SwapInterval(display, interval) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

/// Terminate an EGL display connection.
///
/// # Parameters
///
/// * `display` - Specifies the EGL display connection to terminate.
pub fn terminate(display: EGLDisplay) -> Result<bool, Error> {
    match unsafe { egl::Terminate(display) } {
        1 => Ok(true),
        _ => Err(Error::new()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{egl, so};

    fn egl_init() -> (so::SharedObject, egl::EGLDisplay) {
        #[cfg(feature = "hi3559av100")]
        {
            println!("NOTE: You must start HiFB before run tests!!!");
        }
        let so = so::SharedObject::load("libEGL.so");
        egl::load_with(|s| so.get_proc_address(s));
        egl::load_with_priv(|s| so.get_proc_address(s));
        let dpy = crate::get_default_display().unwrap();
        let mut major: egl::EGLint = 3;
        let mut minor: egl::EGLint = 2;
        assert!(crate::initialize(dpy, &mut major, &mut minor).unwrap());
        (so, dpy)
    }

    #[cfg(feature = "hi3559av100")]
    #[test]
    fn test_bind_api() {
        let (_so, _dpy) = egl_init();
        assert!(crate::bind_api(egl::OPENGL_ES_API).unwrap());
    }

    #[cfg(feature = "hi3559av100")]
    #[test]
    fn test_query_string() {
        let (_so, dpy) = egl_init();
        let apis = crate::query_string(dpy, egl::CLIENT_APIS as egl::EGLint).unwrap();
        assert_eq!(apis, "OpenGL_ES");
    }
}
