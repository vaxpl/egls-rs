use crate::{egl, egl::EGLNativePixmapType};
use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub enum PixmapFormat {
    // ABGR_1_5_5_5
    Abgr1555,
    Abgr1555Afbc,
    // ABGR_4_4_4_4
    Abgr4444,
    Abgr4444Afbc,
    // ABGR_8_8_8_8
    Abgr8888,
    Abgr8888Afbc,
    Abgr8888AfbcSplitBlk,
    Abgr8888AfbcWideBlk,
    // ARGB_1_5_5_5
    Argb1555,
    // ARGB_4_4_4_4
    Argb4444,
    // ARGB_8_8_8_8
    Argb8888,
    Argb8888Afbc,
    Argb8888AfbcSplitBlk,
    Argb8888AfbcWideBlk,
    // ARGB 32Bits
    Argb8888UI,
    // BGR_5_6_5
    Bgr565,
    Bgr565Afbc,
    Bgr565AfbcSplitBlk,
    Bgr565AfbcWideBlk,
    // BGR_8_8_8
    Bgr888,
    Bgr888Afbc,
    Bgr888AfbcSplitBlk,
    Bgr888AfbcWideBlk,
    // BGRA_4_4_4_4
    Bgra4444,
    // BGRA_5_5_5_1
    Bgra5551,
    // BGRA_8_8_8_8
    Bgra8888,
    Bgra8888Afbc,
    Bgra8888AfbcSplitBlk,
    Bgra8888AfbcWideBlk,
    // BGRX_8_8_8_8
    Bgrx8888,
    // Luminance
    L8,
    // NV12
    Nv12Bt601Narrow,
    Nv12Bt601Wide,
    Nv12Bt709Narrow,
    Nv12Bt709Wide,
    // NV16
    Nv16Bt601Narrow,
    Nv16Bt601Wide,
    Nv16Bt709Narrow,
    Nv16Bt709Wide,
    // NV16
    Nv21Bt601Narrow,
    Nv21Bt601Wide,
    Nv21Bt709Narrow,
    Nv21Bt709Wide,
    // P
    P010,
    P210,
    // Bayer
    R8,
    R16,
    Rg8,
    Rg16,
    // RGB_5_6_5
    Rgb565,
    Rgb565Afbc,
    Rgb565AfbcSplitBlk,
    Rgb565AfbcWideBlk,
    // RGB_8_8_8
    Rgb888,
    Rgb888Afbc,
    Rgb888AfbcSplitBlk,
    Rgb888AfbcWideBlk,
    // RGBA_4_4_4_4
    Rgba4444,
    // RGBA_5_5_5_1
    Rgba5551,
    // RGBA_8_8_8_8
    Rgba8888,
    Rgba8888Afbc,
    Rgba8888AfbcSplitBlk,
    Rgba8888AfbcWideBlk,
    // RGBX_8_8_8_8
    Rgbx8888,
    Rgbx8888Afbc,
    Rgbx8888AfbcSplitBlk,
    Rgbx8888AfbcWideBlk,
    // s[ABGR/RGBA/XBGR]_8_8_8_8
    Sabgr8888,
    Sargb8888,
    Sxbgr8888,
    // XBGR_8_8_8_8
    Xbgr8888,
    Xbgr8888Afbc,
    Xbgr8888AfbcSplitBlk,
    Xbgr8888AfbcWideBlk,
    // XRGB_8_8_8_8
    Xrgb8888,
    Xrgb8888Afbc,
    Xrgb8888AfbcSplitBlk,
    Xrgb8888AfbcWideBlk,
    // Y
    Y0l2,
    Y210,
    Y410,
    // YV12
    Yv12Bt601Narrow,
    Yv12Bt601Wide,
    Yv12Bt709Narrow,
    Yv12Bt709Wide,
    // YUYV
    YuyvBt601Narrow,
    YuyvBt601Wide,
    YuyvBt709Narrow,
    YuyvBt709Wide,
    // YUV420 8Bit
    Yuv4208BitBt601NarrowAfbc,
    Yuv4208BitBt601WideAfbc,
    Yuv4208BitBt709NarrowAfbc,
    Yuv4208BitBt709WideAfbc,
    Yuv4208BitBt601NarrowAfbcSplitBlk,
    Yuv4208BitBt601WideAfbcSplitBlk,
    Yuv4208BitBt709NarrowAfbcSplitBlk,
    Yuv4208BitBt709WideAfbcSplitBlk,
    Yuv4208BitBt601NarrowAfbcWideBlk,
    Yuv4208BitBt601WideAfbcWideBlk,
    Yuv4208BitBt709NarrowAfbcWideBlk,
    Yuv4208BitBt709WideAfbcWideBlk,
    // YUV422 8Bit
    Yuv4228BitBt601NarrowAfbc,
    Yuv4228BitBt601WideAfbc,
    Yuv4228BitBt709NarrowAfbc,
    Yuv4228BitBt709WideAfbc,
    Yuv4228BitBt601NarrowAfbcSplitBlk,
    Yuv4228BitBt601WideAfbcSplitBlk,
    Yuv4228BitBt709NarrowAfbcSplitBlk,
    Yuv4228BitBt709WideAfbcSplitBlk,
    Yuv4228BitBt601NarrowAfbcWideBlk,
    Yuv4228BitBt601WideAfbcWideBlk,
    Yuv4228BitBt709NarrowAfbcWideBlk,
    Yuv4228BitBt709WideAfbcWideBlk,
    // YUV420 10Bit
    Yuv42010BitAfbc,
    Yuv42010BitAfbcWideBlk,
    // YUV422 10Bit
    Yuv42210BitAfbc,
    Yuv42210BitAfbcWideBlk,
    //
    Butt,
}

impl Default for PixmapFormat {
    fn default() -> Self {
        PixmapFormat::Argb8888
    }
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
        Self { native, finalizer }
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

#[derive(Default)]
pub struct PixmapBuilder<'a, 'b> {
    phy_addr: u64,
    width: isize,
    height: isize,
    format: PixmapFormat,
    strides: Option<&'b [usize]>,
    finalizer: Cell<Option<Finalizer<'a>>>,
}

impl<'a, 'b> PixmapBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self {
            phy_addr: 0,
            width: 0,
            height: 0,
            format: Default::default(),
            strides: None,
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

    pub fn with_strides(&mut self, strides: &'b [usize]) -> &mut Self {
        self.strides = Some(strides);
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
        let r = if let Some(s) = self.strides {
            Pixmap::new(
                egl::NativePixmap::with_strides(
                    self.phy_addr,
                    self.width,
                    self.height,
                    self.format,
                    s,
                ),
                self.finalizer.replace(None),
            )
        } else {
            Pixmap::new(
                egl::NativePixmap::new(self.phy_addr, self.width, self.height, self.format),
                self.finalizer.replace(None),
            )
        };
        Ok(r)
    }
}
