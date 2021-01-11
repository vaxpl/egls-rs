use crate::pixmap::PixmapFormat;

pub const KHRONOS_SUPPORT_INT64: u32 = 1;
pub const KHRONOS_SUPPORT_FLOAT: u32 = 1;
pub const KHRONOS_MAX_ENUM: u32 = 2147483647;

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

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum khronos_boolean_enum_t {
    KHRONOS_FALSE = 0,
    KHRONOS_TRUE = 1,
    KHRONOS_BOOLEAN_ENUM_FORCE_SIZE = 2147483647,
}

pub type EGLNativeDisplayType = usize;
pub type EGLNativePixmapType = usize;
pub type EGLNativeWindowType = usize;

#[derive(Clone, Debug)]
pub struct NativePixmap {
    id: NativePixmapType,
    valid: bool,
}

impl NativePixmap {
    pub fn new(_phy_addr: u64, _width: isize, _height: isize, _format: PixmapFormat) -> Self {
        unsafe {
            Self {
                id: 0,
                valid: false,
            }
        }
    }

    pub fn with_strides(_phy_addr: u64, _width: isize, _height: isize, _format: PixmapFormat, strides: &[usize]) -> Self {
        Self {
            id: 0,
            valid: false,
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
mod dummy {

}
