#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#[cfg(feature = "hi3559av100")]
use lazy_static::lazy_static;

use std::convert::TryInto;

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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct fbdev_window {
    pub width: ::std::os::raw::c_ushort,
    pub height: ::std::os::raw::c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mem_handle {
    pub fd: ::std::os::raw::c_int,
    _ump_mem_handle: u32,
}

impl std::fmt::Debug for mem_handle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { write!(f, "mem_handle {{ fd: {}}}", self.fd) }
    }
}

impl std::default::Default for mem_handle {
    fn default() -> Self {
        mem_handle { fd: -1 }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum fbdev_pixmap_format {
    PIXMAP_FORMAT_BGR565 = 2077032448,
    PIXMAP_FORMAT_RGB565 = 2077032449,
    PIXMAP_FORMAT_BGR565_AFBC = 2077032450,
    PIXMAP_FORMAT_RGB565_AFBC = 2077032451,
    PIXMAP_FORMAT_BGR565_AFBC_SPLITBLK = 2077032452,
    PIXMAP_FORMAT_RGB565_AFBC_SPLITBLK = 2077032453,
    PIXMAP_FORMAT_BGR565_AFBC_WIDEBLK = 2077032454,
    PIXMAP_FORMAT_RGB565_AFBC_WIDEBLK = 2077032455,
    PIXMAP_FORMAT_ABGR8888 = 2077032456,
    PIXMAP_FORMAT_ARGB8888 = 2077032457,
    PIXMAP_FORMAT_ARGB8888UI = 2077032458,
    PIXMAP_FORMAT_BGRA8888 = 2077032459,
    PIXMAP_FORMAT_RGBA8888 = 2077032460,
    PIXMAP_FORMAT_ABGR8888_AFBC = 2077032461,
    PIXMAP_FORMAT_XBGR8888_AFBC = 2077032462,
    PIXMAP_FORMAT_ARGB8888_AFBC = 2077032463,
    PIXMAP_FORMAT_BGRA8888_AFBC = 2077032464,
    PIXMAP_FORMAT_RGBA8888_AFBC = 2077032465,
    PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK = 2077032466,
    PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK = 2077032467,
    PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK = 2077032468,
    PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK = 2077032469,
    PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK = 2077032470,
    PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK_WIDEBLK = 2077032471,
    PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK_WIDEBLK = 2077032472,
    PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK_WIDEBLK = 2077032473,
    PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK_WIDEBLK = 2077032474,
    PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK_WIDEBLK = 2077032475,
    PIXMAP_FORMAT_XBGR8888 = 2077032476,
    PIXMAP_FORMAT_XRGB8888 = 2077032477,
    PIXMAP_FORMAT_BGRX8888 = 2077032478,
    PIXMAP_FORMAT_RGBX8888 = 2077032479,
    PIXMAP_FORMAT_BGR888 = 2077032480,
    PIXMAP_FORMAT_RGB888 = 2077032481,
    PIXMAP_FORMAT_BGR888_AFBC = 2077032482,
    PIXMAP_FORMAT_RGB888_AFBC = 2077032483,
    PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK = 2077032484,
    PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK = 2077032485,
    PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK_WIDEBLK = 2077032486,
    PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK_WIDEBLK = 2077032487,
    PIXMAP_FORMAT_ABGR4444 = 2077032488,
    PIXMAP_FORMAT_ABGR4444_AFBC = 2077032489,
    PIXMAP_FORMAT_ARGB4444 = 2077032490,
    PIXMAP_FORMAT_BGRA4444 = 2077032491,
    PIXMAP_FORMAT_RGBA4444 = 2077032492,
    PIXMAP_FORMAT_ABGR1555 = 2077032493,
    PIXMAP_FORMAT_ABGR1555_AFBC = 2077032494,
    PIXMAP_FORMAT_ARGB1555 = 2077032495,
    PIXMAP_FORMAT_BGRA5551 = 2077032496,
    PIXMAP_FORMAT_RGBA5551 = 2077032497,
    PIXMAP_FORMAT_L8 = 2077032498,
    PIXMAP_FORMAT_R8 = 2077032499,
    PIXMAP_FORMAT_RG8 = 2077032500,
    PIXMAP_FORMAT_R16 = 2077032501,
    PIXMAP_FORMAT_RG16 = 2077032502,
    PIXMAP_FORMAT_YV12_BT601_NARROW = 2077032503,
    PIXMAP_FORMAT_YV12_BT601_WIDE = 2077032504,
    PIXMAP_FORMAT_YV12_BT709_NARROW = 2077032505,
    PIXMAP_FORMAT_YV12_BT709_WIDE = 2077032506,
    PIXMAP_FORMAT_NV12_BT601_NARROW = 2077032507,
    PIXMAP_FORMAT_NV12_BT601_WIDE = 2077032508,
    PIXMAP_FORMAT_NV12_BT709_NARROW = 2077032509,
    PIXMAP_FORMAT_NV12_BT709_WIDE = 2077032510,
    PIXMAP_FORMAT_YUYV_BT601_NARROW = 2077032511,
    PIXMAP_FORMAT_YUYV_BT601_WIDE = 2077032512,
    PIXMAP_FORMAT_YUYV_BT709_NARROW = 2077032513,
    PIXMAP_FORMAT_YUYV_BT709_WIDE = 2077032514,
    PIXMAP_FORMAT_NV21_BT601_NARROW = 2077032515,
    PIXMAP_FORMAT_NV21_BT601_WIDE = 2077032516,
    PIXMAP_FORMAT_NV21_BT709_NARROW = 2077032517,
    PIXMAP_FORMAT_NV21_BT709_WIDE = 2077032518,
    PIXMAP_FORMAT_NV16_BT601_NARROW = 2077032519,
    PIXMAP_FORMAT_NV16_BT601_WIDE = 2077032520,
    PIXMAP_FORMAT_NV16_BT709_NARROW = 2077032521,
    PIXMAP_FORMAT_NV16_BT709_WIDE = 2077032522,
    PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC = 2077032523,
    PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC = 2077032524,
    PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC = 2077032525,
    PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC = 2077032526,
    PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC = 2077032527,
    PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC = 2077032528,
    PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC = 2077032529,
    PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC = 2077032530,
    PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_SPLITBLK = 2077032531,
    PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_SPLITBLK = 2077032532,
    PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_SPLITBLK = 2077032533,
    PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_SPLITBLK = 2077032534,
    PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_WIDEBLK = 2077032535,
    PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_WIDEBLK = 2077032536,
    PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_WIDEBLK = 2077032537,
    PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_WIDEBLK = 2077032538,
    PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC_WIDEBLK = 2077032539,
    PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC_WIDEBLK = 2077032540,
    PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC_WIDEBLK = 2077032541,
    PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC_WIDEBLK = 2077032542,
    PIXMAP_FORMAT_Y0L2 = 2077032543,
    PIXMAP_FORMAT_P010 = 2077032544,
    PIXMAP_FORMAT_P210 = 2077032545,
    PIXMAP_FORMAT_Y210 = 2077032546,
    PIXMAP_FORMAT_Y410 = 2077032547,
    PIXMAP_FORMAT_YUV420_10BIT_AFBC = 2077032548,
    PIXMAP_FORMAT_YUV422_10BIT_AFBC = 2077032549,
    PIXMAP_FORMAT_YUV420_10BIT_AFBC_WIDEBLK = 2077032550,
    PIXMAP_FORMAT_YUV422_10BIT_AFBC_WIDEBLK = 2077032551,
    PIXMAP_FORMAT_sABGR8888 = 2077032552,
    PIXMAP_FORMAT_sARGB8888 = 2077032553,
    PIXMAP_FORMAT_sXBGR8888 = 2077032554,
    PIXMAP_FORMAT_BUTT = 2077032555,
}

impl From<u64> for fbdev_pixmap_format {
    fn from(value: u64) -> Self {
        use fbdev_pixmap_format::*;

        match value {
            2077032448 => PIXMAP_FORMAT_BGR565,
            2077032449 => PIXMAP_FORMAT_RGB565,
            2077032450 => PIXMAP_FORMAT_BGR565_AFBC,
            2077032451 => PIXMAP_FORMAT_RGB565_AFBC,
            2077032452 => PIXMAP_FORMAT_BGR565_AFBC_SPLITBLK,
            2077032453 => PIXMAP_FORMAT_RGB565_AFBC_SPLITBLK,
            2077032454 => PIXMAP_FORMAT_BGR565_AFBC_WIDEBLK,
            2077032455 => PIXMAP_FORMAT_RGB565_AFBC_WIDEBLK,
            2077032456 => PIXMAP_FORMAT_ABGR8888,
            2077032457 => PIXMAP_FORMAT_ARGB8888,
            2077032458 => PIXMAP_FORMAT_ARGB8888UI,
            2077032459 => PIXMAP_FORMAT_BGRA8888,
            2077032460 => PIXMAP_FORMAT_RGBA8888,
            2077032461 => PIXMAP_FORMAT_ABGR8888_AFBC,
            2077032462 => PIXMAP_FORMAT_XBGR8888_AFBC,
            2077032463 => PIXMAP_FORMAT_ARGB8888_AFBC,
            2077032464 => PIXMAP_FORMAT_BGRA8888_AFBC,
            2077032465 => PIXMAP_FORMAT_RGBA8888_AFBC,
            2077032466 => PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK,
            2077032467 => PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK,
            2077032468 => PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK,
            2077032469 => PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK,
            2077032470 => PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK,
            2077032471 => PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK_WIDEBLK,
            2077032472 => PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK_WIDEBLK,
            2077032473 => PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK_WIDEBLK,
            2077032474 => PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK_WIDEBLK,
            2077032475 => PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK_WIDEBLK,
            2077032476 => PIXMAP_FORMAT_XBGR8888,
            2077032477 => PIXMAP_FORMAT_XRGB8888,
            2077032478 => PIXMAP_FORMAT_BGRX8888,
            2077032479 => PIXMAP_FORMAT_RGBX8888,
            2077032480 => PIXMAP_FORMAT_BGR888,
            2077032481 => PIXMAP_FORMAT_RGB888,
            2077032482 => PIXMAP_FORMAT_BGR888_AFBC,
            2077032483 => PIXMAP_FORMAT_RGB888_AFBC,
            2077032484 => PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK,
            2077032485 => PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK,
            2077032486 => PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK_WIDEBLK,
            2077032487 => PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK_WIDEBLK,
            2077032488 => PIXMAP_FORMAT_ABGR4444,
            2077032489 => PIXMAP_FORMAT_ABGR4444_AFBC,
            2077032490 => PIXMAP_FORMAT_ARGB4444,
            2077032491 => PIXMAP_FORMAT_BGRA4444,
            2077032492 => PIXMAP_FORMAT_RGBA4444,
            2077032493 => PIXMAP_FORMAT_ABGR1555,
            2077032494 => PIXMAP_FORMAT_ABGR1555_AFBC,
            2077032495 => PIXMAP_FORMAT_ARGB1555,
            2077032496 => PIXMAP_FORMAT_BGRA5551,
            2077032497 => PIXMAP_FORMAT_RGBA5551,
            2077032498 => PIXMAP_FORMAT_L8,
            2077032499 => PIXMAP_FORMAT_R8,
            2077032500 => PIXMAP_FORMAT_RG8,
            2077032501 => PIXMAP_FORMAT_R16,
            2077032502 => PIXMAP_FORMAT_RG16,
            2077032503 => PIXMAP_FORMAT_YV12_BT601_NARROW,
            2077032504 => PIXMAP_FORMAT_YV12_BT601_WIDE,
            2077032505 => PIXMAP_FORMAT_YV12_BT709_NARROW,
            2077032506 => PIXMAP_FORMAT_YV12_BT709_WIDE,
            2077032507 => PIXMAP_FORMAT_NV12_BT601_NARROW,
            2077032508 => PIXMAP_FORMAT_NV12_BT601_WIDE,
            2077032509 => PIXMAP_FORMAT_NV12_BT709_NARROW,
            2077032510 => PIXMAP_FORMAT_NV12_BT709_WIDE,
            2077032511 => PIXMAP_FORMAT_YUYV_BT601_NARROW,
            2077032512 => PIXMAP_FORMAT_YUYV_BT601_WIDE,
            2077032513 => PIXMAP_FORMAT_YUYV_BT709_NARROW,
            2077032514 => PIXMAP_FORMAT_YUYV_BT709_WIDE,
            2077032515 => PIXMAP_FORMAT_NV21_BT601_NARROW,
            2077032516 => PIXMAP_FORMAT_NV21_BT601_WIDE,
            2077032517 => PIXMAP_FORMAT_NV21_BT709_NARROW,
            2077032518 => PIXMAP_FORMAT_NV21_BT709_WIDE,
            2077032519 => PIXMAP_FORMAT_NV16_BT601_NARROW,
            2077032520 => PIXMAP_FORMAT_NV16_BT601_WIDE,
            2077032521 => PIXMAP_FORMAT_NV16_BT709_NARROW,
            2077032522 => PIXMAP_FORMAT_NV16_BT709_WIDE,
            2077032523 => PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC,
            2077032524 => PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC,
            2077032525 => PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC,
            2077032526 => PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC,
            2077032527 => PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC,
            2077032528 => PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC,
            2077032529 => PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC,
            2077032530 => PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC,
            2077032531 => PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_SPLITBLK,
            2077032532 => PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_SPLITBLK,
            2077032533 => PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_SPLITBLK,
            2077032534 => PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_SPLITBLK,
            2077032535 => PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_WIDEBLK,
            2077032536 => PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_WIDEBLK,
            2077032537 => PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_WIDEBLK,
            2077032538 => PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_WIDEBLK,
            2077032539 => PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC_WIDEBLK,
            2077032540 => PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC_WIDEBLK,
            2077032541 => PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC_WIDEBLK,
            2077032542 => PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC_WIDEBLK,
            2077032543 => PIXMAP_FORMAT_Y0L2,
            2077032544 => PIXMAP_FORMAT_P010,
            2077032545 => PIXMAP_FORMAT_P210,
            2077032546 => PIXMAP_FORMAT_Y210,
            2077032547 => PIXMAP_FORMAT_Y410,
            2077032548 => PIXMAP_FORMAT_YUV420_10BIT_AFBC,
            2077032549 => PIXMAP_FORMAT_YUV422_10BIT_AFBC,
            2077032550 => PIXMAP_FORMAT_YUV420_10BIT_AFBC_WIDEBLK,
            2077032551 => PIXMAP_FORMAT_YUV422_10BIT_AFBC_WIDEBLK,
            2077032552 => PIXMAP_FORMAT_sABGR8888,
            2077032553 => PIXMAP_FORMAT_sARGB8888,
            2077032554 => PIXMAP_FORMAT_sXBGR8888,
            2077032555 => PIXMAP_FORMAT_BUTT,
            _ => unreachable!(),
        }
    }
}

impl Into<u64> for fbdev_pixmap_format {
    fn into(self) -> u64 {
        use fbdev_pixmap_format::*;

        match self {
            PIXMAP_FORMAT_BGR565 => 2077032448,
            PIXMAP_FORMAT_RGB565 => 2077032449,
            PIXMAP_FORMAT_BGR565_AFBC => 2077032450,
            PIXMAP_FORMAT_RGB565_AFBC => 2077032451,
            PIXMAP_FORMAT_BGR565_AFBC_SPLITBLK => 2077032452,
            PIXMAP_FORMAT_RGB565_AFBC_SPLITBLK => 2077032453,
            PIXMAP_FORMAT_BGR565_AFBC_WIDEBLK => 2077032454,
            PIXMAP_FORMAT_RGB565_AFBC_WIDEBLK => 2077032455,
            PIXMAP_FORMAT_ABGR8888 => 2077032456,
            PIXMAP_FORMAT_ARGB8888 => 2077032457,
            PIXMAP_FORMAT_ARGB8888UI => 2077032458,
            PIXMAP_FORMAT_BGRA8888 => 2077032459,
            PIXMAP_FORMAT_RGBA8888 => 2077032460,
            PIXMAP_FORMAT_ABGR8888_AFBC => 2077032461,
            PIXMAP_FORMAT_XBGR8888_AFBC => 2077032462,
            PIXMAP_FORMAT_ARGB8888_AFBC => 2077032463,
            PIXMAP_FORMAT_BGRA8888_AFBC => 2077032464,
            PIXMAP_FORMAT_RGBA8888_AFBC => 2077032465,
            PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK => 2077032466,
            PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK => 2077032467,
            PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK => 2077032468,
            PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK => 2077032469,
            PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK => 2077032470,
            PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK_WIDEBLK => 2077032471,
            PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK_WIDEBLK => 2077032472,
            PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK_WIDEBLK => 2077032473,
            PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK_WIDEBLK => 2077032474,
            PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK_WIDEBLK => 2077032475,
            PIXMAP_FORMAT_XBGR8888 => 2077032476,
            PIXMAP_FORMAT_XRGB8888 => 2077032477,
            PIXMAP_FORMAT_BGRX8888 => 2077032478,
            PIXMAP_FORMAT_RGBX8888 => 2077032479,
            PIXMAP_FORMAT_BGR888 => 2077032480,
            PIXMAP_FORMAT_RGB888 => 2077032481,
            PIXMAP_FORMAT_BGR888_AFBC => 2077032482,
            PIXMAP_FORMAT_RGB888_AFBC => 2077032483,
            PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK => 2077032484,
            PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK => 2077032485,
            PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK_WIDEBLK => 2077032486,
            PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK_WIDEBLK => 2077032487,
            PIXMAP_FORMAT_ABGR4444 => 2077032488,
            PIXMAP_FORMAT_ABGR4444_AFBC => 2077032489,
            PIXMAP_FORMAT_ARGB4444 => 2077032490,
            PIXMAP_FORMAT_BGRA4444 => 2077032491,
            PIXMAP_FORMAT_RGBA4444 => 2077032492,
            PIXMAP_FORMAT_ABGR1555 => 2077032493,
            PIXMAP_FORMAT_ABGR1555_AFBC => 2077032494,
            PIXMAP_FORMAT_ARGB1555 => 2077032495,
            PIXMAP_FORMAT_BGRA5551 => 2077032496,
            PIXMAP_FORMAT_RGBA5551 => 2077032497,
            PIXMAP_FORMAT_L8 => 2077032498,
            PIXMAP_FORMAT_R8 => 2077032499,
            PIXMAP_FORMAT_RG8 => 2077032500,
            PIXMAP_FORMAT_R16 => 2077032501,
            PIXMAP_FORMAT_RG16 => 2077032502,
            PIXMAP_FORMAT_YV12_BT601_NARROW => 2077032503,
            PIXMAP_FORMAT_YV12_BT601_WIDE => 2077032504,
            PIXMAP_FORMAT_YV12_BT709_NARROW => 2077032505,
            PIXMAP_FORMAT_YV12_BT709_WIDE => 2077032506,
            PIXMAP_FORMAT_NV12_BT601_NARROW => 2077032507,
            PIXMAP_FORMAT_NV12_BT601_WIDE => 2077032508,
            PIXMAP_FORMAT_NV12_BT709_NARROW => 2077032509,
            PIXMAP_FORMAT_NV12_BT709_WIDE => 2077032510,
            PIXMAP_FORMAT_YUYV_BT601_NARROW => 2077032511,
            PIXMAP_FORMAT_YUYV_BT601_WIDE => 2077032512,
            PIXMAP_FORMAT_YUYV_BT709_NARROW => 2077032513,
            PIXMAP_FORMAT_YUYV_BT709_WIDE => 2077032514,
            PIXMAP_FORMAT_NV21_BT601_NARROW => 2077032515,
            PIXMAP_FORMAT_NV21_BT601_WIDE => 2077032516,
            PIXMAP_FORMAT_NV21_BT709_NARROW => 2077032517,
            PIXMAP_FORMAT_NV21_BT709_WIDE => 2077032518,
            PIXMAP_FORMAT_NV16_BT601_NARROW => 2077032519,
            PIXMAP_FORMAT_NV16_BT601_WIDE => 2077032520,
            PIXMAP_FORMAT_NV16_BT709_NARROW => 2077032521,
            PIXMAP_FORMAT_NV16_BT709_WIDE => 2077032522,
            PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC => 2077032523,
            PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC => 2077032524,
            PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC => 2077032525,
            PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC => 2077032526,
            PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC => 2077032527,
            PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC => 2077032528,
            PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC => 2077032529,
            PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC => 2077032530,
            PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_SPLITBLK => 2077032531,
            PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_SPLITBLK => 2077032532,
            PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_SPLITBLK => 2077032533,
            PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_SPLITBLK => 2077032534,
            PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_WIDEBLK => 2077032535,
            PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_WIDEBLK => 2077032536,
            PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_WIDEBLK => 2077032537,
            PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_WIDEBLK => 2077032538,
            PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC_WIDEBLK => 2077032539,
            PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC_WIDEBLK => 2077032540,
            PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC_WIDEBLK => 2077032541,
            PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC_WIDEBLK => 2077032542,
            PIXMAP_FORMAT_Y0L2 => 2077032543,
            PIXMAP_FORMAT_P010 => 2077032544,
            PIXMAP_FORMAT_P210 => 2077032545,
            PIXMAP_FORMAT_Y210 => 2077032546,
            PIXMAP_FORMAT_Y410 => 2077032547,
            PIXMAP_FORMAT_YUV420_10BIT_AFBC => 2077032548,
            PIXMAP_FORMAT_YUV422_10BIT_AFBC => 2077032549,
            PIXMAP_FORMAT_YUV420_10BIT_AFBC_WIDEBLK => 2077032550,
            PIXMAP_FORMAT_YUV422_10BIT_AFBC_WIDEBLK => 2077032551,
            PIXMAP_FORMAT_sABGR8888 => 2077032552,
            PIXMAP_FORMAT_sARGB8888 => 2077032553,
            PIXMAP_FORMAT_sXBGR8888 => 2077032554,
            PIXMAP_FORMAT_BUTT => 2077032555,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct linux_pixmap_plane {
    pub stride: khronos_usize_t,
    pub size: khronos_usize_t,
    pub offset: khronos_usize_t,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct linux_pixmap {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub planes: [linux_pixmap_plane; 3usize],
    pub pixmap_format: u64,
    pub handles: [mem_handle; 3usize],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct dummy_display {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub bytes_per_pixel: ::std::os::raw::c_int,
    pub red_mask: ::std::os::raw::c_ulong,
    pub green_mask: ::std::os::raw::c_ulong,
    pub blue_mask: ::std::os::raw::c_ulong,
    pub alpha_mask: ::std::os::raw::c_ulong,
    pub front_buffer: *mut ::std::os::raw::c_uchar,
}

impl std::default::Default for dummy_display {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[cfg(feature = "hi3559av100")]
pub const DBE_COMMAND_WRAP: u32 = 2148549888;

#[cfg(feature = "hi3559av100")]
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct hidbe_ioctl_wrap {
    dbe_phyaddr: u64,
    dbe_size: u64,
}

pub type EGLNativeWindowType = *mut fbdev_window;
pub type EGLNativePixmapType = i32;
pub type EGLNativeDisplayType = *mut ::std::os::raw::c_void;
pub type NativeWindowType = EGLNativeWindowType;
pub type NativePixmapType = EGLNativePixmapType;
pub type NativeDisplayType = EGLNativeDisplayType;
pub type EGLint = i32;

include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));

#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn lookup_pixmap_ID_mapping(id: i32) -> *mut linux_pixmap {
    __gl_imports::mem::transmute::<_, extern "system" fn(i32) -> *mut linux_pixmap>(
        storage_priv::lookup_pixmap_ID_mapping.f,
    )(id)
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn create_pixmap_ID_mapping(pixmap: *mut linux_pixmap) -> i32 {
    __gl_imports::mem::transmute::<_, extern "system" fn(*mut linux_pixmap) -> i32>(
        storage_priv::create_pixmap_ID_mapping.f,
    )(pixmap)
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn destroy_pixmap_ID_mapping(id: i32) -> ::std::os::raw::c_uint {
    __gl_imports::mem::transmute::<_, extern "system" fn(i32) -> ::std::os::raw::c_uint>(
        storage_priv::destroy_pixmap_ID_mapping.f,
    )(id)
}

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
        lookup_pixmap_ID_mapping::load_with(&mut *loadfn);
        create_pixmap_ID_mapping::load_with(&mut *loadfn);
        destroy_pixmap_ID_mapping::load_with(&mut *loadfn);
    }

    inner(&mut loadfn)
}

#[cfg(feature = "hi3559av100")]
struct DmaBufferExporter {
    fd: std::os::raw::c_int,
}

#[cfg(feature = "hi3559av100")]
impl DmaBufferExporter {
    fn new() -> Self {
        unsafe {
            let cstr = std::ffi::CStr::from_bytes_with_nul(b"/dev/hi_dbe\0").unwrap();
            let fd = libc::open(cstr.as_ptr(), libc::O_RDWR);
            assert!(fd > 0, "Failed to open: `{:?}`!", cstr);
            Self { fd }
        }
    }

    fn wrap_fd(&self, phy_addr: u64, size: u64) -> std::os::raw::c_int {
        unsafe {
            let mut wrap: hidbe_ioctl_wrap = Default::default();
            wrap.dbe_phyaddr = phy_addr;
            wrap.dbe_size = size;
            let dmabuf_fd = libc::ioctl(self.fd, DBE_COMMAND_WRAP.try_into().unwrap(), &wrap);
            assert!(
                dmabuf_fd > 0,
                "Failed to wrap with dma for {} @ {:X}!",
                phy_addr,
                size
            );
            dmabuf_fd
        }
    }
}

#[cfg(feature = "hi3559av100")]
impl Drop for DmaBufferExporter {
    fn drop(&mut self) {
        if self.fd > 0 {
            unsafe {
                libc::close(self.fd);
            }
            self.fd = -1;
        }
    }
}

#[cfg(feature = "hi3559av100")]
lazy_static! {
    static ref DBE: DmaBufferExporter = DmaBufferExporter::new();
}

#[cfg(feature = "hi3559av100")]
pub unsafe fn hi_dbe_wrap_dma_buf_fd(phy_addr: u64, size: u64) -> std::os::raw::c_int {
    let mut wrap: hidbe_ioctl_wrap = Default::default();
    let mut dmabuf_fd: std::os::raw::c_int = -1;
    let cstr = std::ffi::CStr::from_bytes_with_nul(b"/dev/hi_dbe\0").unwrap();
    let fd = libc::open(cstr.as_ptr(), libc::O_RDWR);
    assert!(fd > 0, "Failed to open: `{:?}`!", cstr);
    wrap.dbe_phyaddr = phy_addr;
    wrap.dbe_size = size;
    dmabuf_fd = libc::ioctl(fd, DBE_COMMAND_WRAP.try_into().unwrap(), &wrap);
    libc::close(fd);
    assert!(
        dmabuf_fd > 0,
        "Failed to wrap with dma for {} @ {:X}!",
        phy_addr,
        size
    );
    dmabuf_fd
}

impl linux_pixmap {
    #[cfg(feature = "hi3559av100")]
    pub fn new(phy_addr: u64, width: isize, height: isize, format: u64) -> Self {
        use fbdev_pixmap_format::*;

        let w: khronos_usize_t = width.try_into().unwrap();
        let h: khronos_usize_t = height.try_into().unwrap();
        let mut dma: linux_pixmap = Default::default();
        dma.width = width.try_into().unwrap();
        dma.height = height.try_into().unwrap();
        dma.pixmap_format = format;

        match fbdev_pixmap_format::from(format) {
            PIXMAP_FORMAT_BGR565
            | PIXMAP_FORMAT_RGB565
            | PIXMAP_FORMAT_BGR565_AFBC
            | PIXMAP_FORMAT_RGB565_AFBC
            | PIXMAP_FORMAT_BGR565_AFBC_SPLITBLK
            | PIXMAP_FORMAT_RGB565_AFBC_SPLITBLK
            | PIXMAP_FORMAT_BGR565_AFBC_WIDEBLK
            | PIXMAP_FORMAT_RGB565_AFBC_WIDEBLK
            | PIXMAP_FORMAT_ABGR4444
            | PIXMAP_FORMAT_ABGR4444_AFBC
            | PIXMAP_FORMAT_ARGB4444
            | PIXMAP_FORMAT_BGRA4444
            | PIXMAP_FORMAT_RGBA4444
            | PIXMAP_FORMAT_ABGR1555
            | PIXMAP_FORMAT_ABGR1555_AFBC
            | PIXMAP_FORMAT_ARGB1555
            | PIXMAP_FORMAT_BGRA5551
            | PIXMAP_FORMAT_RGBA5551 => {
                dma.planes[0].stride = w * 2;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                unsafe {
                    dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
                }
            }
            PIXMAP_FORMAT_BGR888
            | PIXMAP_FORMAT_RGB888
            | PIXMAP_FORMAT_BGR888_AFBC
            | PIXMAP_FORMAT_RGB888_AFBC
            | PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK_WIDEBLK
            | PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK_WIDEBLK => {
                dma.planes[0].stride = w * 3;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                unsafe {
                    dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
                }
            }
            PIXMAP_FORMAT_ABGR8888
            | PIXMAP_FORMAT_ARGB8888
            | PIXMAP_FORMAT_ARGB8888UI
            | PIXMAP_FORMAT_BGRA8888
            | PIXMAP_FORMAT_RGBA8888
            | PIXMAP_FORMAT_ABGR8888_AFBC
            | PIXMAP_FORMAT_XBGR8888_AFBC
            | PIXMAP_FORMAT_ARGB8888_AFBC
            | PIXMAP_FORMAT_BGRA8888_AFBC
            | PIXMAP_FORMAT_RGBA8888_AFBC
            | PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK_WIDEBLK
            | PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK_WIDEBLK
            | PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK_WIDEBLK
            | PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK_WIDEBLK
            | PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK_WIDEBLK
            | PIXMAP_FORMAT_XBGR8888
            | PIXMAP_FORMAT_XRGB8888
            | PIXMAP_FORMAT_BGRX8888
            | PIXMAP_FORMAT_RGBX8888 => {
                dma.planes[0].stride = w * 4;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                unsafe {
                    dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
                }
            }
            PIXMAP_FORMAT_NV21_BT709_WIDE => {
                dma.planes[0].stride = w;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = w;
                dma.planes[1].size = dma.planes[0].stride * h / 2;
                dma.planes[1].offset = dma.planes[0].size;
                unsafe {
                    dma.handles[0].fd =
                        DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                    dma.handles[1].fd = dma.handles[0].fd;
                }
            }
            PIXMAP_FORMAT_P010 => {
                dma.planes[0].stride = w * 2;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = w * 2;
                dma.planes[1].size = dma.planes[1].stride * h / 2;
                dma.planes[1].offset = dma.planes[0].size;
                unsafe {
                    dma.handles[0].fd =
                        DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                    dma.handles[1].fd = dma.handles[0].fd;
                }
            }
            _ => unreachable!(),
        }
        dma
    }
    #[cfg(not(feature = "hi3559av100"))]
    pub fn new(_phy_addr: u64, _width: isize, _height: isize, _format: u64) -> Self {
        unimplemented!()
    }
}

#[cfg(feature = "hi3559av100")]
impl Drop for linux_pixmap {
    fn drop(&mut self) {
        for a in &self.handles {
            unsafe {
                libc::close(a.fd);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NativePixmap {
    dma: *mut linux_pixmap,
    id: NativePixmapType,
    valid: bool,
}

impl NativePixmap {
    pub fn new(phy_addr: u64, width: isize, height: isize, format: u64) -> Self {
        unsafe {
            let dma = Box::new(linux_pixmap::new(phy_addr, width, height, format));
            let dma = Box::into_raw(dma);
            let id = create_pixmap_ID_mapping(dma);
            Self {
                dma: dma,
                id: id,
                valid: true,
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
            dma: std::ptr::null_mut(),
            id: 0,
            valid: false,
        }
    }
}

impl Drop for NativePixmap {
    fn drop(&mut self) {
        if self.valid {
            unsafe {
                destroy_pixmap_ID_mapping(self.id);
                drop(Box::from_raw(self.dma));
            }
        }
    }
}

// Re-Export types
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_layout_fbdev_window() {
        assert_eq!(
            ::std::mem::size_of::<fbdev_window>(),
            4usize,
            concat!("Size of: ", stringify!(fbdev_window))
        );
        assert_eq!(
            ::std::mem::align_of::<fbdev_window>(),
            2usize,
            concat!("Alignment of ", stringify!(fbdev_window))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<fbdev_window>())).width as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(fbdev_window),
                "::",
                stringify!(width)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<fbdev_window>())).height as *const _ as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(fbdev_window),
                "::",
                stringify!(height)
            )
        );
    }

    #[test]
    fn test_layout_mem_handle() {
        assert_eq!(
            ::std::mem::size_of::<mem_handle>(),
            4usize,
            concat!("Size of: ", stringify!(mem_handle))
        );
        assert_eq!(
            ::std::mem::align_of::<mem_handle>(),
            4usize,
            concat!("Alignment of ", stringify!(mem_handle))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<mem_handle>())).fd as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(mem_handle),
                "::",
                stringify!(fd)
            )
        );
    }

    #[test]
    fn test_layout_linux_pixmap_plane() {
        assert_eq!(
            ::std::mem::size_of::<linux_pixmap_plane>(),
            24usize,
            concat!("Size of: ", stringify!(linux_pixmap_plane))
        );
        assert_eq!(
            ::std::mem::align_of::<linux_pixmap_plane>(),
            8usize,
            concat!("Alignment of ", stringify!(linux_pixmap_plane))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap_plane>())).stride as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap_plane),
                "::",
                stringify!(stride)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap_plane>())).size as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap_plane),
                "::",
                stringify!(size)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap_plane>())).offset as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap_plane),
                "::",
                stringify!(offset)
            )
        );
    }

    #[test]
    fn test_layout_linux_pixmap() {
        assert_eq!(
            ::std::mem::size_of::<linux_pixmap>(),
            104usize,
            concat!("Size of: ", stringify!(linux_pixmap))
        );
        assert_eq!(
            ::std::mem::align_of::<linux_pixmap>(),
            8usize,
            concat!("Alignment of ", stringify!(linux_pixmap))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap>())).width as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap),
                "::",
                stringify!(width)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap>())).height as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap),
                "::",
                stringify!(height)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap>())).planes as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap),
                "::",
                stringify!(planes)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap>())).pixmap_format as *const _ as usize },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap),
                "::",
                stringify!(pixmap_format)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<linux_pixmap>())).handles as *const _ as usize },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(linux_pixmap),
                "::",
                stringify!(handles)
            )
        );
    }

    #[test]
    fn test_layout_dummy_display() {
        assert_eq!(
            ::std::mem::size_of::<dummy_display>(),
            56usize,
            concat!("Size of: ", stringify!(dummy_display))
        );
        assert_eq!(
            ::std::mem::align_of::<dummy_display>(),
            8usize,
            concat!("Alignment of ", stringify!(dummy_display))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<dummy_display>())).width as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(width)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<dummy_display>())).height as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(height)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<dummy_display>())).bytes_per_pixel as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(bytes_per_pixel)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<dummy_display>())).red_mask as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(red_mask)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<dummy_display>())).green_mask as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(green_mask)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<dummy_display>())).blue_mask as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(blue_mask)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<dummy_display>())).alpha_mask as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(alpha_mask)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<dummy_display>())).front_buffer as *const _ as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(dummy_display),
                "::",
                stringify!(front_buffer)
            )
        );
    }

    #[cfg(feature = "hi3559av100")]
    #[test]
    fn test_layout_hidbe_ioctl_wrap() {
        assert_eq!(
            ::std::mem::size_of::<hidbe_ioctl_wrap>(),
            16usize,
            concat!("Size of: ", stringify!(hidbe_ioctl_wrap))
        );
        assert_eq!(
            ::std::mem::align_of::<hidbe_ioctl_wrap>(),
            8usize,
            concat!("Alignment of ", stringify!(hidbe_ioctl_wrap))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<hidbe_ioctl_wrap>())).dbe_phyaddr as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(hidbe_ioctl_wrap),
                "::",
                stringify!(dbe_phyaddr)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<hidbe_ioctl_wrap>())).dbe_size as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(hidbe_ioctl_wrap),
                "::",
                stringify!(dbe_size)
            )
        );
    }

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
