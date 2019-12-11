use crate::{ffi::*, so::SharedObject};
use std::convert::TryInto;
use std::sync::Arc;
use std::vec::Vec;

/// Environment of the EGL instance.
#[derive(Clone, Debug)]
struct Environment {
    so: SharedObject,
    display: EGLDisplay,
    context: EGLContext,
    surface: EGLSurface,
}

impl Environment {
    pub fn new(
        so: SharedObject,
        display: EGLDisplay,
        context: EGLContext,
        surface: EGLSurface,
    ) -> Self {
        Self {
            so,
            display,
            context,
            surface,
        }
    }

    pub fn make_current(&self) {
        crate::make_current(self.display, self.surface, self.surface, self.context).unwrap();
    }

    pub fn swap_buffers(&self) {
        crate::swap_buffers(self.display, self.surface).unwrap();
    }
}

#[derive(Clone, Default, Debug)]
struct EnvironmentOptions {
    native_window: fbdev_window,
    config_attribs: Vec<EGLint>,
    context_attribs: Vec<EGLint>,
    offscreen: bool,
}

#[derive(Clone, Default, Debug)]
struct EnvironmentBuilder {
    options: EnvironmentOptions,
}

impl EnvironmentBuilder {
    pub fn defaults() -> Self {
        let mut options: EnvironmentOptions = Default::default();
        // 1280x720
        options.native_window.width = 1280;
        options.native_window.height = 720;
        // RGBA32
        options.config_attribs.extend_from_slice(&[
            SAMPLES as EGLint,
            0,
            RED_SIZE as EGLint,
            5,
            GREEN_SIZE as EGLint,
            5,
            BLUE_SIZE as EGLint,
            5,
            ALPHA_SIZE as EGLint,
            1,
            DEPTH_SIZE as EGLint,
            0,
            STENCIL_SIZE as EGLint,
            0,
            SURFACE_TYPE as EGLint,
            WINDOW_BIT as EGLint,
            NONE as EGLint,
        ]);
        // OpenGL ES 3.x
        options.context_attribs.extend_from_slice(&[
            CONTEXT_CLIENT_VERSION as EGLint,
            3,
            NONE as EGLint,
        ]);
        // Offscreen = false
        options.offscreen = false;

        Self { options: options }
    }

    pub fn window_size(mut self, width: usize, height: usize) -> Self {
        self.options.native_window.width = width.try_into().unwrap();
        self.options.native_window.height = height.try_into().unwrap();
        self
    }

    pub fn build(&mut self) -> Arc<Environment> {
        let so = SharedObject::load("libEGL.so");
        load_with(|s| so.get_proc_address(s));
        load_with_priv(|s| so.get_proc_address(s));
        let mut configs: EGLConfig = std::ptr::null_mut();
        let mut num_config: EGLint = 0;
        let display = crate::get_default_display().unwrap();
        crate::bind_api(crate::OPENGL_ES_API).unwrap();
        crate::initialize_default(display).unwrap();
        crate::choose_config(
            display,
            self.options.config_attribs.as_ptr(),
            &mut configs,
            1,
            &mut num_config,
        )
        .unwrap();
        let surface = crate::create_window_surface(
            display,
            configs,
            &mut self.options.native_window,
            std::ptr::null_mut(),
        )
        .unwrap();
        let context = crate::create_context(
            display,
            configs,
            std::ptr::null_mut(),
            self.options.context_attribs.as_ptr(),
        )
        .unwrap();
        crate::make_current(display, surface, surface, context).unwrap();
        Arc::new(Environment::new(so, display, context, surface))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_builder() {
        let env = EnvironmentBuilder::defaults()
            .window_size(1920, 1080)
            .build();
    }
}
