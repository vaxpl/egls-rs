use x11::xlib::{Display, Pixmap, Window};

pub type khronos_int32_t = i32;
pub type khronos_uint32_t = u32;
pub type khronos_int64_t = i64;
pub type khronos_uint64_t = u64;
pub type khronos_int8_t = ::std::os::raw::c_schar;
pub type khronos_uint8_t = ::std::os::raw::c_uchar;
pub type khronos_int16_t = ::std::os::raw::c_short;
pub type khronos_uint16_t = ::std::os::raw::c_ushort;
pub type khronos_intptr_t = ::std::os::raw::c_long;
pub type khronos_uintptr_t = ::std::os::raw::c_ulong;
pub type khronos_ssize_t = ::std::os::raw::c_long;
pub type khronos_usize_t = ::std::os::raw::c_ulong;
pub type khronos_float_t = f32;
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_stime_nanoseconds_t = khronos_int64_t;

pub type EGLNativeDisplayType = *mut Display;
pub type EGLNativePixmapType = Pixmap;
pub type EGLNativeWindowType = Window;

#[derive(Clone, Debug)]
pub struct NativePixmap {
    id: NativePixmapType,
    valid: bool,
}

impl NativePixmap {
    pub fn new(_phy_addr: u64, _width: isize, _height: isize, _format: u64) -> Self {
        unsafe {
            Self {
                id: 0,
                valid: false,
            }
        }
    }

    pub fn id(&self) -> NativePixmapType {
        self.id
    }
}

impl Default for NativePixmap {
    fn default() -> Self {
        Self {
            id: 0,
            valid: false,
        }
    }
}

impl Drop for NativePixmap {
    fn drop(&mut self) {
        if self.valid {
        }
    }
}

#[cfg(test)]
mod x11 {

}
