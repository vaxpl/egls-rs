use crate::pixmap::PixmapFormat;
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

impl From<PixmapFormat> for fbdev_pixmap_format {
    fn from(value: PixmapFormat) -> Self {
        use PixmapFormat::*;
        use fbdev_pixmap_format::*;

        match value {
            Bgr565 => PIXMAP_FORMAT_BGR565,
            Rgb565 => PIXMAP_FORMAT_RGB565,
            Bgr565Afbc => PIXMAP_FORMAT_BGR565_AFBC,
            Rgb565Afbc => PIXMAP_FORMAT_RGB565_AFBC,
            Bgr565AfbcSplitBlk => PIXMAP_FORMAT_BGR565_AFBC_SPLITBLK,
            Rgb565AfbcSplitBlk => PIXMAP_FORMAT_RGB565_AFBC_SPLITBLK,
            Bgr565AfbcWideBlk => PIXMAP_FORMAT_BGR565_AFBC_WIDEBLK,
            Rgb565AfbcWideBlk => PIXMAP_FORMAT_RGB565_AFBC_WIDEBLK,
            Abgr8888 => PIXMAP_FORMAT_ABGR8888,
            Argb8888 => PIXMAP_FORMAT_ARGB8888,
            Argb8888UI => PIXMAP_FORMAT_ARGB8888UI,
            Bgra8888 => PIXMAP_FORMAT_BGRA8888,
            Rgba8888 => PIXMAP_FORMAT_RGBA8888,
            Abgr8888Afbc => PIXMAP_FORMAT_ABGR8888_AFBC,
            Xbgr8888Afbc => PIXMAP_FORMAT_XBGR8888_AFBC,
            Argb8888Afbc => PIXMAP_FORMAT_ARGB8888_AFBC,
            Bgra8888Afbc => PIXMAP_FORMAT_BGRA8888_AFBC,
            Rgba8888Afbc => PIXMAP_FORMAT_RGBA8888_AFBC,
            Abgr8888AfbcSplitBlk => PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK,
            Xbgr8888AfbcSplitBlk => PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK,
            Argb8888AfbcSplitBlk => PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK,
            Bgra8888AfbcSplitBlk => PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK,
            Rgba8888AfbcSplitBlk => PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK,
            Abgr8888AfbcWideBlk => PIXMAP_FORMAT_ABGR8888_AFBC_SPLITBLK_WIDEBLK,
            Xbgr8888AfbcWideBlk => PIXMAP_FORMAT_XBGR8888_AFBC_SPLITBLK_WIDEBLK,
            Argb8888AfbcWideBlk => PIXMAP_FORMAT_ARGB8888_AFBC_SPLITBLK_WIDEBLK,
            Bgra8888AfbcWideBlk => PIXMAP_FORMAT_BGRA8888_AFBC_SPLITBLK_WIDEBLK,
            Rgba8888AfbcWideBlk => PIXMAP_FORMAT_RGBA8888_AFBC_SPLITBLK_WIDEBLK,
            Xbgr8888 => PIXMAP_FORMAT_XBGR8888,
            Xrgb8888 => PIXMAP_FORMAT_XRGB8888,
            Bgrx8888 => PIXMAP_FORMAT_BGRX8888,
            Rgbx8888 => PIXMAP_FORMAT_RGBX8888,
            Bgr888 => PIXMAP_FORMAT_BGR888,
            Rgb888 => PIXMAP_FORMAT_RGB888,
            Bgr888Afbc => PIXMAP_FORMAT_BGR888_AFBC,
            Rgb888Afbc => PIXMAP_FORMAT_RGB888_AFBC,
            Bgr888AfbcSplitBlk => PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK,
            Rgb888AfbcSplitBlk => PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK,
            Bgr888AfbcWideBlk => PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK_WIDEBLK,
            Rgb888AfbcWideBlk => PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK_WIDEBLK,
            Abgr4444 => PIXMAP_FORMAT_ABGR4444,
            Abgr4444Afbc => PIXMAP_FORMAT_ABGR4444_AFBC,
            Argb4444 => PIXMAP_FORMAT_ARGB4444,
            Bgra4444 => PIXMAP_FORMAT_BGRA4444,
            Rgba4444 => PIXMAP_FORMAT_RGBA4444,
            Abgr1555 => PIXMAP_FORMAT_ABGR1555,
            Abgr1555Afbc => PIXMAP_FORMAT_ABGR1555_AFBC,
            Argb1555 => PIXMAP_FORMAT_ARGB1555,
            Bgra5551 => PIXMAP_FORMAT_BGRA5551,
            Rgba5551 => PIXMAP_FORMAT_RGBA5551,
            L8 => PIXMAP_FORMAT_L8,
            R8 => PIXMAP_FORMAT_R8,
            Rg8 => PIXMAP_FORMAT_RG8,
            R16 => PIXMAP_FORMAT_R16,
            Rg16 => PIXMAP_FORMAT_RG16,
            Yv12Bt601Narrow => PIXMAP_FORMAT_YV12_BT601_NARROW,
            Yv12Bt601Wide => PIXMAP_FORMAT_YV12_BT601_WIDE,
            Yv12Bt709Narrow => PIXMAP_FORMAT_YV12_BT709_NARROW,
            Yv12Bt709Wide => PIXMAP_FORMAT_YV12_BT709_WIDE,
            Nv12Bt601Narrow => PIXMAP_FORMAT_NV12_BT601_NARROW,
            Nv12Bt601Wide => PIXMAP_FORMAT_NV12_BT601_WIDE,
            Nv12Bt709Narrow => PIXMAP_FORMAT_NV12_BT709_NARROW,
            Nv12Bt709Wide => PIXMAP_FORMAT_NV12_BT709_WIDE,
            YuyvBt601Narrow => PIXMAP_FORMAT_YUYV_BT601_NARROW,
            YuyvBt601Wide => PIXMAP_FORMAT_YUYV_BT601_WIDE,
            YuyvBt709Narrow => PIXMAP_FORMAT_YUYV_BT709_NARROW,
            YuyvBt709Wide => PIXMAP_FORMAT_YUYV_BT709_WIDE,
            Nv21Bt601Narrow => PIXMAP_FORMAT_NV21_BT601_NARROW,
            Nv21Bt601Wide => PIXMAP_FORMAT_NV21_BT601_WIDE,
            Nv21Bt709Narrow => PIXMAP_FORMAT_NV21_BT709_NARROW,
            Nv21Bt709Wide => PIXMAP_FORMAT_NV21_BT709_WIDE,
            Nv16Bt601Narrow => PIXMAP_FORMAT_NV16_BT601_NARROW,
            Nv16Bt601Wide => PIXMAP_FORMAT_NV16_BT601_WIDE,
            Nv16Bt709Narrow => PIXMAP_FORMAT_NV16_BT709_NARROW,
            Nv16Bt709Wide => PIXMAP_FORMAT_NV16_BT709_WIDE,
            Yuv4208BitBt601NarrowAfbc => PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC,
            Yuv4208BitBt601WideAfbc => PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC,
            Yuv4208BitBt709NarrowAfbc => PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC,
            Yuv4208BitBt709WideAfbc => PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC,
            Yuv4228BitBt601NarrowAfbc => PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC,
            Yuv4228BitBt601WideAfbc => PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC,
            Yuv4228BitBt709NarrowAfbc => PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC,
            Yuv4228BitBt709WideAfbc => PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC,
            Yuv4208BitBt601NarrowAfbcSplitBlk  => PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_SPLITBLK,
            Yuv4208BitBt601WideAfbcSplitBlk  => PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_SPLITBLK,
            Yuv4208BitBt709NarrowAfbcSplitBlk  => PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_SPLITBLK,
            Yuv4208BitBt709WideAfbcSplitBlk  => PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_SPLITBLK,
            Yuv4208BitBt601NarrowAfbcWideBlk => PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC_WIDEBLK,
            Yuv4208BitBt601WideAfbcWideBlk => PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC_WIDEBLK,
            Yuv4208BitBt709NarrowAfbcWideBlk => PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC_WIDEBLK,
            Yuv4208BitBt709WideAfbcWideBlk => PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC_WIDEBLK,
            Yuv4228BitBt601NarrowAfbcWideBlk => PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC_WIDEBLK,
            Yuv4228BitBt601WideAfbcWideBlk => PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC_WIDEBLK,
            Yuv4228BitBt709NarrowAfbcWideBlk => PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC_WIDEBLK,
            Yuv4228BitBt709WideAfbcWideBlk => PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC_WIDEBLK,
            Y0l2 => PIXMAP_FORMAT_Y0L2,
            P010 => PIXMAP_FORMAT_P010,
            P210 => PIXMAP_FORMAT_P210,
            Y210 => PIXMAP_FORMAT_Y210,
            Y410 => PIXMAP_FORMAT_Y410,
            Yuv42010BitAfbc => PIXMAP_FORMAT_YUV420_10BIT_AFBC,
            Yuv42210BitAfbc => PIXMAP_FORMAT_YUV422_10BIT_AFBC,
            Yuv42010BitAfbcWideBlk => PIXMAP_FORMAT_YUV420_10BIT_AFBC_WIDEBLK,
            Yuv42210BitAfbcWideBlk => PIXMAP_FORMAT_YUV422_10BIT_AFBC_WIDEBLK,
            Sabgr8888 => PIXMAP_FORMAT_sABGR8888,
            Sargb8888 => PIXMAP_FORMAT_sARGB8888,
            Sxbgr8888 => PIXMAP_FORMAT_sXBGR8888,
            Butt => PIXMAP_FORMAT_BUTT,
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
            let wrap = hidbe_ioctl_wrap {
                dbe_phyaddr: phy_addr,
                dbe_size: size,
            };
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

impl linux_pixmap {
    #[cfg(feature = "plat-mali-fbdev")]
    pub fn new(phy_addr: u64, width: isize, height: isize, format: PixmapFormat) -> Self {
        use fbdev_pixmap_format::*;

        let w: khronos_usize_t = width.try_into().unwrap();
        let h: khronos_usize_t = height.try_into().unwrap();
        let mut dma = linux_pixmap {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            pixmap_format: fbdev_pixmap_format::from(format).into(),
            ..Default::default()
        };
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
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
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
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
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
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
            }
            PIXMAP_FORMAT_NV21_BT601_NARROW
            | PIXMAP_FORMAT_NV21_BT601_WIDE
            | PIXMAP_FORMAT_NV21_BT709_NARROW
            | PIXMAP_FORMAT_NV21_BT709_WIDE => {
                dma.planes[0].stride = w;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = w;
                dma.planes[1].size = dma.planes[1].stride * h / 2;
                dma.planes[1].offset = dma.planes[0].size;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                dma.handles[1].fd = unsafe { dma.handles[0].fd };
            }
            PIXMAP_FORMAT_NV16_BT601_NARROW
            | PIXMAP_FORMAT_NV16_BT601_WIDE
            | PIXMAP_FORMAT_NV16_BT709_NARROW
            | PIXMAP_FORMAT_NV16_BT709_WIDE => {
                dma.planes[0].stride = w;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = w;
                dma.planes[1].size = dma.planes[1].stride * h;
                dma.planes[1].offset = dma.planes[0].size;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                dma.handles[1].fd = unsafe { dma.handles[0].fd };
            }
            PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC_WIDEBLK
            | PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC_WIDEBLK
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC_WIDEBLK
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC_WIDEBLK => {
                dma.planes[0].stride = w * 2;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
            }
            PIXMAP_FORMAT_P010 => {
                dma.planes[0].stride = w * 2;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = w * 2;
                dma.planes[1].size = dma.planes[1].stride * h / 2;
                dma.planes[1].offset = dma.planes[0].size;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                dma.handles[1].fd = unsafe { dma.handles[0].fd };
            }
            _ => unreachable!(),
        }
        dma
    }

    #[cfg(not(feature = "hi3559av100"))]
    pub fn new(_phy_addr: u64, _width: isize, _height: isize, _format: u64) -> Self {
        unimplemented!()
    }

    #[cfg(feature = "plat-mali-fbdev")]
    pub fn with_strides(phy_addr: u64, width: isize, height: isize, format: PixmapFormat, strides: &[usize]) -> Self {
        use fbdev_pixmap_format::*;

        let _w: khronos_usize_t = width.try_into().unwrap();
        let h: khronos_usize_t = height.try_into().unwrap();
        let mut dma = linux_pixmap {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            pixmap_format: fbdev_pixmap_format::from(format).into(),
            ..Default::default()
        };
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
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
            }
            PIXMAP_FORMAT_BGR888
            | PIXMAP_FORMAT_RGB888
            | PIXMAP_FORMAT_BGR888_AFBC
            | PIXMAP_FORMAT_RGB888_AFBC
            | PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK
            | PIXMAP_FORMAT_BGR888_AFBC_SPLITBLK_WIDEBLK
            | PIXMAP_FORMAT_RGB888_AFBC_SPLITBLK_WIDEBLK => {
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
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
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
            }
            | PIXMAP_FORMAT_L8 => {
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
            }
            PIXMAP_FORMAT_NV21_BT601_NARROW
            | PIXMAP_FORMAT_NV21_BT601_WIDE
            | PIXMAP_FORMAT_NV21_BT709_NARROW
            | PIXMAP_FORMAT_NV21_BT709_WIDE => {
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = strides[1] as u64;
                dma.planes[1].size = dma.planes[1].stride * h / 2;
                dma.planes[1].offset = dma.planes[0].size;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                dma.handles[1].fd = unsafe { dma.handles[0].fd };
            }
            PIXMAP_FORMAT_NV16_BT601_NARROW
            | PIXMAP_FORMAT_NV16_BT601_WIDE
            | PIXMAP_FORMAT_NV16_BT709_NARROW
            | PIXMAP_FORMAT_NV16_BT709_WIDE => {
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = strides[1] as u64;
                dma.planes[1].size = dma.planes[1].stride * h;
                dma.planes[1].offset = dma.planes[0].size;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                dma.handles[1].fd = unsafe { dma.handles[0].fd };
            }
            PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC
            | PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC_WIDEBLK
            | PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC_WIDEBLK
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC_WIDEBLK
            | PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC_WIDEBLK => {
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size);
            }
            PIXMAP_FORMAT_P010 => {
                dma.planes[0].stride = strides[0] as u64;
                dma.planes[0].size = dma.planes[0].stride * h;
                dma.planes[0].offset = 0;
                dma.planes[1].stride = strides[1] as u64;
                dma.planes[1].size = dma.planes[1].stride * h / 2;
                dma.planes[1].offset = dma.planes[0].size;
                dma.handles[0].fd = DBE.wrap_fd(phy_addr, dma.planes[0].size + dma.planes[1].size);
                dma.handles[1].fd = unsafe { dma.handles[0].fd };
            }
            _ => unreachable!(),
        }
        dma
    }

    #[cfg(not(feature = "hi3559av100"))]
    pub fn with_strides<'r>(_phy_addr: u64, _width: isize, _height: isize, _format: u64, _strides: &'r [usize]) -> Self {
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
    pub fn new(phy_addr: u64, width: isize, height: isize, format: PixmapFormat) -> Self {
        unsafe {
            let dma = Box::new(linux_pixmap::new(phy_addr, width, height, format));
            let dma = Box::into_raw(dma);
            let id = create_pixmap_ID_mapping(dma);
            Self {
                dma,
                id,
                valid: true,
            }
        }
    }

    pub fn with_strides(phy_addr: u64, width: isize, height: isize, format: PixmapFormat, strides: &[usize]) -> Self {
        unsafe {
            let dma = Box::new(linux_pixmap::with_strides(phy_addr, width, height, format, strides));
            let dma = Box::into_raw(dma);
            let id = create_pixmap_ID_mapping(dma);
            Self {
                dma,
                id,
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

#[cfg(test)]
mod mali_fbdev {
    use super::*;

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
}
