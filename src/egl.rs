#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::unreadable_literal)]

#[cfg(feature = "plat-mali-fbdev")]
include! {"platform/mali-fbdev.rs"}

#[cfg(feature = "plat-x11")]
include! {"platform/x11.rs"}

#[cfg(not(any(feature = "plat-mali-fbdev", feature = "plat-x11")))]
include! {"platform/dummy.rs"}

pub type NativeWindowType = EGLNativeWindowType;
pub type NativePixmapType = EGLNativePixmapType;
pub type NativeDisplayType = EGLNativeDisplayType;
pub type EGLint = i32;

include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));

#[cfg(feature = "plat-mali-fbdev")]
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn lookup_pixmap_ID_mapping(id: i32) -> *mut linux_pixmap {
    __gl_imports::mem::transmute::<_, extern "system" fn(i32) -> *mut linux_pixmap>(
        storage_priv::lookup_pixmap_ID_mapping.f,
    )(id)
}

#[cfg(feature = "plat-mali-fbdev")]
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn create_pixmap_ID_mapping(pixmap: *mut linux_pixmap) -> i32 {
    __gl_imports::mem::transmute::<_, extern "system" fn(*mut linux_pixmap) -> i32>(
        storage_priv::create_pixmap_ID_mapping.f,
    )(pixmap)
}

#[cfg(feature = "plat-mali-fbdev")]
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn destroy_pixmap_ID_mapping(id: i32) -> ::std::os::raw::c_uint {
    __gl_imports::mem::transmute::<_, extern "system" fn(i32) -> ::std::os::raw::c_uint>(
        storage_priv::destroy_pixmap_ID_mapping.f,
    )(id)
}

#[cfg(feature = "plat-mali-fbdev")]
mod storage_priv {
    use super::FnPtr;
    use super::__gl_imports::raw;

    pub static mut lookup_pixmap_ID_mapping: FnPtr = FnPtr {
        f: super::missing_fn_panic as *const raw::c_void,
        is_loaded: false,
    };

    pub static mut create_pixmap_ID_mapping: FnPtr = FnPtr {
        f: super::missing_fn_panic as *const raw::c_void,
        is_loaded: false,
    };

    pub static mut destroy_pixmap_ID_mapping: FnPtr = FnPtr {
        f: super::missing_fn_panic as *const raw::c_void,
        is_loaded: false,
    };
}

#[cfg(feature = "plat-mali-fbdev")]
#[allow(non_snake_case)]
pub mod lookup_pixmap_ID_mapping {
    use super::FnPtr;
    use super::__gl_imports::raw;
    use super::{metaloadfn, storage_priv};

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage_priv::lookup_pixmap_ID_mapping.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
    where
        F: FnMut(&'static str) -> *const raw::c_void,
    {
        unsafe {
            storage_priv::lookup_pixmap_ID_mapping =
                FnPtr::new(metaloadfn(&mut loadfn, "egl_lookup_pixmap_ID_mapping", &[]))
        }
    }
}

#[cfg(feature = "plat-mali-fbdev")]
#[allow(non_snake_case)]
pub mod create_pixmap_ID_mapping {
    use super::FnPtr;
    use super::__gl_imports::raw;
    use super::{metaloadfn, storage_priv};

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage_priv::create_pixmap_ID_mapping.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
    where
        F: FnMut(&'static str) -> *const raw::c_void,
    {
        unsafe {
            storage_priv::create_pixmap_ID_mapping =
                FnPtr::new(metaloadfn(&mut loadfn, "egl_create_pixmap_ID_mapping", &[]))
        }
    }
}

#[cfg(feature = "plat-mali-fbdev")]
#[allow(non_snake_case)]
pub mod destroy_pixmap_ID_mapping {
    use super::FnPtr;
    use super::__gl_imports::raw;
    use super::{metaloadfn, storage_priv};

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage_priv::destroy_pixmap_ID_mapping.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
    where
        F: FnMut(&'static str) -> *const raw::c_void,
    {
        unsafe {
            storage_priv::destroy_pixmap_ID_mapping = FnPtr::new(metaloadfn(
                &mut loadfn,
                "egl_destroy_pixmap_ID_mapping",
                &[],
            ))
        }
    }
}

/// Load each EGL private symbol using a custom load function. This allows for the
/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
/// ~~~ignore
/// gl::load_with_priv(|s| glfw.get_proc_address(s));
/// ~~~
#[allow(dead_code)]
pub fn load_with_priv<F>(mut loadfn: F)
where
    F: FnMut(&'static str) -> *const __gl_imports::raw::c_void,
{
    #[inline(never)]
    fn inner(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void) {
        #[cfg(feature = "plat-mali-fbdev")]
        {
            lookup_pixmap_ID_mapping::load_with(&mut *loadfn);
            create_pixmap_ID_mapping::load_with(&mut *loadfn);
            destroy_pixmap_ID_mapping::load_with(&mut *loadfn);
        }
    }

    inner(&mut loadfn)
}

// Re-Export types
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_load_with() {
        use crate::so::SharedObject;

        let so = SharedObject::load("libEGL.so");
        load_with(|s| so.get_proc_address(s));
        load_with_priv(|s| so.get_proc_address(s));

        unsafe {
            let dpy = GetDisplay(DEFAULT_DISPLAY);
            Initialize(dpy, std::ptr::null_mut(), std::ptr::null_mut());
            let apis = std::ffi::CStr::from_ptr(QueryString(dpy, CLIENT_APIS.try_into().unwrap()));
            println!("EGL_CLIENT_APIS={:?}", apis);
        }
    }
}
