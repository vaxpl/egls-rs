use crate::{egl, Environment, Error, Pixmap};

/// Callback for Image cleanup native resources.
pub type Finalizer<'a> = Box<dyn Fn(&Image) + 'a>;

/// Native Handle of the EGLImage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum NativeHandle {
    EglImage(egl::EGLImage),
    EglImageKHR(egl::EGLImageKHR),
}

impl Default for NativeHandle {
    fn default() -> Self {
        NativeHandle::EglImage(std::ptr::null())
    }
}

impl Into<usize> for NativeHandle {
    fn into(self) -> usize {
        match self {
            NativeHandle::EglImage(v) => v as usize,
            NativeHandle::EglImageKHR(v) => v as usize,
        }
    }
}

impl Into<*const std::ffi::c_void> for NativeHandle {
    fn into(self) -> *const std::ffi::c_void {
        match self {
            NativeHandle::EglImage(v) => v as *const std::ffi::c_void,
            NativeHandle::EglImageKHR(v) => v as *const std::ffi::c_void,
        }
    }
}

/// Type of resource used as the image source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Target {
    /// Used for GL 2D texture images
    GlTexture2D = egl::GL_TEXTURE_2D as isize,
    /// Used for the +X face of GL cubemap texture images
    GlTextureCubeMapPositiveX = egl::GL_TEXTURE_CUBE_MAP_POSITIVE_X as isize,
    /// Used for the -X face of GL cubemap texture images
    GlTextureCubeMapNegativeX = egl::GL_TEXTURE_CUBE_MAP_NEGATIVE_X as isize,
    /// Used for the +Y face of GL cubemap texture images
    GlTextureCubeMapPositiveY = egl::GL_TEXTURE_CUBE_MAP_POSITIVE_Y as isize,
    /// Used for the -Y face of GL cubemap texture images
    GlTextureCubeMapNegativeY = egl::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y as isize,
    /// Used for the +Z face of GL cubemap texture images
    GlTextureCubeMapPositiveZ = egl::GL_TEXTURE_CUBE_MAP_POSITIVE_Z as isize,
    /// Used for the -Z face of GL cubemap texture images
    GlTextureCubeMapNegativeZ = egl::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z as isize,
    /// Used for OpenGL and OpenGL ES 3D texture images
    GlTexture3D = egl::GL_TEXTURE_3D as isize,
    /// Used for OpenGL and OpenGL ES renderbuffer images
    GlRenderBuffer = egl::GL_RENDERBUFFER as isize,
    /// Used for EGLNativePixmapType objects
    NativePixmapKHR = egl::NATIVE_PIXMAP_KHR as isize,
}

impl Default for Target {
    fn default() -> Self {
        Target::GlTexture2D
    }
}

impl Into<egl::EGLenum> for Target {
    fn into(self) -> egl::EGLenum {
        match self {
            Target::GlTexture2D => egl::GL_TEXTURE_2D,
            Target::GlTextureCubeMapPositiveX => egl::GL_TEXTURE_CUBE_MAP_POSITIVE_X,
            Target::GlTextureCubeMapNegativeX => egl::GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
            Target::GlTextureCubeMapPositiveY => egl::GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
            Target::GlTextureCubeMapNegativeY => egl::GL_TEXTURE_CUBE_MAP_NEGATIVE_Y,
            Target::GlTextureCubeMapPositiveZ => egl::GL_TEXTURE_CUBE_MAP_POSITIVE_Z,
            Target::GlTextureCubeMapNegativeZ => egl::GL_TEXTURE_CUBE_MAP_NEGATIVE_Z,
            Target::GlTexture3D => egl::GL_TEXTURE_3D,
            Target::GlRenderBuffer => egl::GL_RENDERBUFFER,
            Target::NativePixmapKHR => egl::NATIVE_PIXMAP_KHR,
        }
    }
}

/// The Buffer of the Image.
#[derive(Debug)]
pub enum ImageBuffer<'a> {
    EglClientBuffer(egl::EGLClientBuffer),
    Pixmap(Pixmap<'a>),
}

impl<'a> Default for ImageBuffer<'a> {
    fn default() -> Self {
        ImageBuffer::EglClientBuffer(std::ptr::null())
    }
}

/// Image with native resources.
#[allow(dead_code)]
pub struct Image<'a> {
    display: egl::EGLDisplay,
    context: egl::EGLContext,
    target: Target,
    buffer: ImageBuffer<'a>,
    finalizer: Option<Finalizer<'a>>,
    native: NativeHandle,
}

// unsafe impl<'a> Send for Image<'a> {}
// unsafe impl<'a> Sync for Image<'a> {}

impl<'a> Image<'a> {
    pub fn new(
        env: &'_ Environment,
        target: Target,
        buffer: ImageBuffer<'a>,
        finalizer: Option<Finalizer<'a>>,
    ) -> Self {
        let mut _native: NativeHandle = Default::default();
        match buffer {
            ImageBuffer::EglClientBuffer(client_buffer) => {
                let a = crate::create_image(
                    env.get_display(),
                    env.get_context(),
                    target.into(),
                    client_buffer,
                    None,
                )
                .unwrap();
                _native = NativeHandle::EglImage(a);
            }
            ImageBuffer::Pixmap(ref pixmap) => {
                let a = crate::create_image(
                    env.get_display(),
                    env.get_context(),
                    target.into(),
                    pixmap.id() as egl::EGLClientBuffer,
                    None,
                )
                .unwrap();
                _native = NativeHandle::EglImageKHR(a);
            }
        }
        Self {
            display: env.get_display(),
            context: env.get_context(),
            target,
            buffer,
            finalizer,
            native: _native,
        }
    }

    pub fn new_khr(
        env: &'_ Environment,
        target: Target,
        buffer: ImageBuffer<'a>,
        finalizer: Option<Finalizer<'a>>,
    ) -> Self {
        let mut ctx = env.get_context();
        let mut _native: NativeHandle = Default::default();
        if let Target::NativePixmapKHR = target {
            ctx = egl::NO_CONTEXT;
        }
        match buffer {
            ImageBuffer::EglClientBuffer(client_buffer) => {
                let a = crate::create_image_khr(
                    env.get_display(),
                    ctx,
                    target as egl::EGLenum,
                    client_buffer,
                    None,
                )
                .unwrap();
                _native = NativeHandle::EglImageKHR(a);
            }
            ImageBuffer::Pixmap(ref pixmap) => {
                let a = crate::create_image_khr(
                    env.get_display(),
                    ctx,
                    target as egl::EGLenum,
                    pixmap.id() as egl::EGLClientBuffer,
                    None,
                )
                .unwrap();
                _native = NativeHandle::EglImageKHR(a);
            }
        }
        Self {
            display: env.get_display(),
            context: env.get_context(),
            target,
            buffer,
            finalizer,
            native: _native,
        }
    }

    pub fn id(&self) -> usize {
        self.native.into()
    }

    pub fn native(&self) -> NativeHandle {
        self.native
    }
}

impl<'a> Default for Image<'a> {
    fn default() -> Self {
        Self {
            display: egl::NO_DISPLAY,
            context: egl::NO_CONTEXT,
            target: Default::default(),
            buffer: Default::default(),
            finalizer: None,
            native: Default::default(),
        }
    }
}

impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        if let Some(ref f) = self.finalizer {
            (f)(self);
        }
        match self.native {
            NativeHandle::EglImage(v) => {
                crate::destroy_image(self.display, v).unwrap();
                self.native = Default::default();
            }
            NativeHandle::EglImageKHR(v) => {
                crate::destroy_image_khr(self.display, v).unwrap();
                self.native = Default::default();
            }
        }
    }
}

impl<'a> std::fmt::Debug for Image<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.finalizer {
            Some(ref finalizer) => write!(
                f,
                "Image {{ native: {:?}, buffer: {:?}, finalizer: {:p} }}",
                self.native, self.buffer, finalizer
            ),
            None => write!(
                f,
                "Image {{ native: {:?}, buffer: {:?}, finalizer: None }}",
                self.native, self.buffer
            ),
        }
    }
}

/// Build an Image with Chain style.
#[derive(Default)]
pub struct ImageBuilder<'a> {
    display: Option<egl::EGLDisplay>,
    context: Option<egl::EGLContext>,
    target: Option<Target>,
    buffer: Option<ImageBuffer<'a>>,
    is_khr: bool,
    finalizer: Option<Finalizer<'a>>,
}

impl<'a> ImageBuilder<'a> {
    pub fn new() -> Self {
        Self {
            display: None,
            context: None,
            target: Some(Default::default()),
            buffer: Some(Default::default()),
            is_khr: false,
            finalizer: None,
        }
    }

    pub fn with_display(mut self, display: egl::EGLDisplay) -> Self {
        self.display = Some(display);
        self
    }

    pub fn with_context(mut self, context: egl::EGLContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_target(mut self, target: Target) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_buffer(mut self, buffer: ImageBuffer<'a>) -> Self {
        // Fix the Target if Use Pixmap Buffer
        match buffer {
            ImageBuffer::Pixmap(_) => {
                self.target = Some(Target::NativePixmapKHR);
            }
            ImageBuffer::EglClientBuffer(_) => {}
        }
        self.buffer = Some(buffer);
        self
    }

    pub fn with_khr(mut self) -> Self {
        self.is_khr = true;
        self
    }

    pub fn with_finalizer<F>(mut self, finalizer: F) -> Self
    where
        F: Fn(&Image) + 'a,
    {
        self.finalizer = Some(Box::new(finalizer));
        self
    }

    pub fn build(self, env: &'_ Environment) -> Result<Image<'a>, Error> {
        if self.is_khr {
            Ok(Image::new_khr(
                env,
                self.target.unwrap_or_default(),
                self.buffer.unwrap_or_default(),
                self.finalizer,
            ))
        } else {
            Ok(Image::new(
                env,
                self.target.unwrap_or_default(),
                self.buffer.unwrap_or_default(),
                self.finalizer,
            ))
        }
    }
}
