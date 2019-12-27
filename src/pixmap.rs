use crate::{egl, egl::EGLNativePixmapType};
use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub enum PixmapFormat {
    Abgr1555 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_ABGR1555 as isize,
    Abgr4444 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_ABGR4444 as isize,
    Abgr8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_ABGR8888 as isize,
    Argb1555 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_ARGB1555 as isize,
    Argb4444 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_ARGB4444 as isize,
    Argb8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_ARGB8888 as isize,
    Argb8888UI = egl::fbdev_pixmap_format::PIXMAP_FORMAT_ARGB8888UI as isize,
    Bgr565 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_BGR565 as isize,
    Bgr888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_BGR888 as isize,
    Bgra4444 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_BGRA4444 as isize,
    Bgra5551 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_BGRA5551 as isize,
    Bgra8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_BGRA8888 as isize,
    Bgrx8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_BGRX8888 as isize,
    L8 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_L8 as isize,
    Nv12Bt601Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV12_BT601_NARROW as isize,
    Nv12Bt601Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV12_BT601_WIDE as isize,
    Nv12Bt709Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV12_BT709_NARROW as isize,
    Nv12Bt709Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV12_BT709_WIDE as isize,
    Nv16Bt601Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV16_BT601_NARROW as isize,
    Nv16Bt601Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV16_BT601_WIDE as isize,
    Nv16Bt709Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV16_BT709_NARROW as isize,
    Nv16Bt709Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV16_BT709_WIDE as isize,
    Nv21Bt601Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV21_BT601_NARROW as isize,
    Nv21Bt601Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV21_BT601_WIDE as isize,
    Nv21Bt709Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV21_BT709_NARROW as isize,
    Nv21Bt709Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_NV21_BT709_WIDE as isize,
    R8 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_R8 as isize,
    R16 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_R16 as isize,
    Rg8 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RG8 as isize,
    Rg16 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RG16 as isize,
    Rgb565 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RGB565 as isize,
    Rgb888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RGB888 as isize,
    Rgba4444 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RGBA4444 as isize,
    Rgba5551 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RGBA5551 as isize,
    Rgba8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RGBA8888 as isize,
    Rgbx8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_RGBX8888 as isize,
    Sabgr8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_sABGR8888 as isize,
    Sargb8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_sARGB8888 as isize,
    Sxbgr8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_sXBGR8888 as isize,
    Xbgr8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_XBGR8888 as isize,
    Xrgb8888 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_XRGB8888 as isize,
    Y0l2 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_Y0L2 as isize,
    Y210 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_Y210 as isize,
    Y410 = egl::fbdev_pixmap_format::PIXMAP_FORMAT_Y410 as isize,
    Yv12Bt601Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YV12_BT601_NARROW as isize,
    Yv12Bt601Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YV12_BT601_WIDE as isize,
    Yv12Bt709Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YV12_BT709_NARROW as isize,
    Yv12Bt709Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YV12_BT709_WIDE as isize,
    YuyvBt601Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUYV_BT601_NARROW as isize,
    YuyvBt601Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUYV_BT601_WIDE as isize,
    YuyvBt709Narrow = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUYV_BT709_NARROW as isize,
    YuyvBt709Wide = egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUYV_BT709_WIDE as isize,
    Yuv4208BitBt601NarrowAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV420_8BIT_BT601_NARROW_AFBC as isize,
    Yuv4208BitBt601WideAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV420_8BIT_BT601_WIDE_AFBC as isize,
    Yuv4208BitBt709NarrowAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV420_8BIT_BT709_NARROW_AFBC as isize,
    Yuv4208BitBt709WideAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV420_8BIT_BT709_WIDE_AFBC as isize,
    Yuv4228BitBt601NarrowAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV422_8BIT_BT601_NARROW_AFBC as isize,
    Yuv4228BitBt601WideAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV422_8BIT_BT601_WIDE_AFBC as isize,
    Yuv4228BitBt709NarrowAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV422_8BIT_BT709_NARROW_AFBC as isize,
    Yuv4228BitBt709WideAfbc =
        egl::fbdev_pixmap_format::PIXMAP_FORMAT_YUV422_8BIT_BT709_WIDE_AFBC as isize,
}

/// Callback for Pixmap cleanup native resources.
pub type Finalizer<'a> = Box<dyn Fn(&Pixmap) + 'a>;

/// Pixmap with native resources.
pub struct Pixmap<'a> {
    native: egl::NativePixmap,
    finalizer: Option<Finalizer<'a>>,
}

unsafe impl<'a> Send for Pixmap<'a> {}
unsafe impl<'a> Sync for Pixmap<'a> {}

impl<'a> Pixmap<'a> {
    pub fn new(native: egl::NativePixmap, finalizer: Option<Finalizer<'a>>) -> Self {
        Self {
            native: native,
            finalizer: finalizer,
        }
    }

    pub fn id(&self) -> EGLNativePixmapType {
        self.native.id()
    }
}

impl<'a> Default for Pixmap<'a> {
    fn default() -> Self {
        Self {
            native: Default::default(),
            finalizer: None,
        }
    }
}

impl<'a> Drop for Pixmap<'a> {
    fn drop(&mut self) {
        if let Some(ref f) = self.finalizer {
            (f)(self);
        }
    }
}

impl<'a> std::fmt::Debug for Pixmap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.finalizer {
            Some(ref finalizer) => write!(
                f,
                "Pixmap {{ native: {:?}, finalizer: {:p} }}",
                self.native, finalizer
            ),
            None => write!(f, "Pixmap {{ native: {:?}, finalizer: None }}", self.native),
        }
    }
}

pub struct PixmapBuilder<'a> {
    phy_addr: u64,
    width: isize,
    height: isize,
    format: PixmapFormat,
    finalizer: Cell<Option<Finalizer<'a>>>,
}

impl<'a> PixmapBuilder<'a> {
    pub fn new() -> Self {
        Self {
            phy_addr: 0,
            width: 0,
            height: 0,
            format: PixmapFormat::Rgba8888,
            finalizer: Cell::new(None),
        }
    }

    pub fn with_phy_addr(&mut self, phy_addr: u64) -> &mut Self {
        self.phy_addr = phy_addr;
        self
    }

    pub fn with_size(&mut self, width: isize, height: isize) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_format(&mut self, format: PixmapFormat) -> &mut Self {
        self.format = format;
        self
    }

    pub fn with_finalizer<F>(&mut self, finalizer: F) -> &mut Self
    where
        F: Fn(&Pixmap) + 'a,
    {
        self.finalizer.replace(Some(Box::new(finalizer)));
        self
    }

    pub fn build(&self) -> Result<Pixmap<'a>, String> {
        Ok(Pixmap::new(
            egl::NativePixmap::new(self.phy_addr, self.width, self.height, self.format as u64),
            self.finalizer.replace(None),
        ))
    }
}
