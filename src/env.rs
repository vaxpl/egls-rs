use crate::{egl, so::SharedObject, EGLConfig, EGLContext, EGLDisplay, EGLSurface, EGLint};
use std::collections::HashMap;
use std::convert::TryInto;
use std::vec::Vec;

/// Environment of the EGL instance.
#[derive(Clone, Debug)]
pub struct Environment {
    so: SharedObject,
    display: EGLDisplay,
    context: EGLContext,
    surface: EGLSurface,
    win: *mut egl::fbdev_window,
}

impl Environment {
    pub fn new(
        so: SharedObject,
        display: EGLDisplay,
        context: EGLContext,
        surface: EGLSurface,
        win: *mut egl::fbdev_window,
    ) -> Self {
        Self {
            so,
            display,
            context,
            surface,
            win,
        }
    }

    pub fn make_current(&self) {
        crate::make_current(self.display, self.surface, self.surface, self.context).unwrap();
    }

    pub fn swap_buffers(&self) {
        crate::swap_buffers(self.display, self.surface).unwrap();
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.win) };
    }
}

#[derive(Clone, Debug)]
struct EnvironmentOptions {
    native_window: egl::fbdev_window,
    config_attribs: HashMap<u32, u32>,
    context_attribs: HashMap<u32, u32>,
    offscreen: bool,
}

impl Default for EnvironmentOptions {
    fn default() -> Self {
        Self {
            native_window: egl::fbdev_window {
                width: 1280,
                height: 720,
            },
            config_attribs: [
                (egl::SAMPLES, 0),
                (egl::RED_SIZE, 8),
                (egl::GREEN_SIZE, 8),
                (egl::BLUE_SIZE, 8),
                (egl::ALPHA_SIZE, 8),
                (egl::DEPTH_SIZE, 0),
                (egl::STENCIL_SIZE, 0),
                (egl::SURFACE_TYPE, egl::WINDOW_BIT),
            ]
            .iter()
            .cloned()
            .collect(),
            context_attribs: [(egl::CONTEXT_CLIENT_VERSION, 3)].iter().cloned().collect(),
            offscreen: false,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct EnvironmentBuilder {
    options: EnvironmentOptions,
}

impl EnvironmentBuilder {
    pub fn defaults() -> Self {
        Self {
            options: Default::default(),
        }
    }

    pub fn with_context_client_version(mut self, version: u32) -> Self {
        self.options
            .context_attribs
            .insert(egl::CONTEXT_CLIENT_VERSION, version);
        self
    }

    pub fn with_rgb_565(mut self) -> Self {
        let attrs = &mut self.options.config_attribs;
        attrs.insert(egl::RED_SIZE, 5);
        attrs.insert(egl::GREEN_SIZE, 6);
        attrs.insert(egl::BLUE_SIZE, 5);
        attrs.insert(egl::ALPHA_SIZE, 0);
        self
    }

    pub fn with_rgb_888(mut self) -> Self {
        let attrs = &mut self.options.config_attribs;
        attrs.insert(egl::RED_SIZE, 8);
        attrs.insert(egl::GREEN_SIZE, 8);
        attrs.insert(egl::BLUE_SIZE, 8);
        attrs.insert(egl::ALPHA_SIZE, 0);
        self
    }

    pub fn with_rgba_5551(mut self) -> Self {
        let attrs = &mut self.options.config_attribs;
        attrs.insert(egl::RED_SIZE, 5);
        attrs.insert(egl::GREEN_SIZE, 5);
        attrs.insert(egl::BLUE_SIZE, 5);
        attrs.insert(egl::ALPHA_SIZE, 1);
        self
    }

    pub fn with_rgba_8888(mut self) -> Self {
        let attrs = &mut self.options.config_attribs;
        attrs.insert(egl::RED_SIZE, 8);
        attrs.insert(egl::GREEN_SIZE, 8);
        attrs.insert(egl::BLUE_SIZE, 8);
        attrs.insert(egl::ALPHA_SIZE, 8);
        self
    }

    pub fn with_samples(mut self, samples: u32) -> Self {
        self.options.config_attribs.insert(egl::SAMPLES, samples);
        self
    }

    pub fn with_window_size(mut self, width: usize, height: usize) -> Self {
        self.options.native_window.width = width.try_into().unwrap();
        self.options.native_window.height = height.try_into().unwrap();
        self
    }

    pub fn build(&mut self) -> Environment {
        let so = SharedObject::load("libEGL.so");
        egl::load_with(|s| so.get_proc_address(s));
        egl::load_with_priv(|s| so.get_proc_address(s));

        let mut configs: EGLConfig = std::ptr::null_mut();
        let mut config_attribs: Vec<EGLint> = Vec::new();
        let mut context_attribs: Vec<EGLint> = Vec::new();
        let mut num_config: EGLint = 0;
        let display = crate::get_default_display().unwrap();

        for (key, val) in self.options.config_attribs.iter() {
            config_attribs.push(*key as EGLint);
            config_attribs.push(*val as EGLint);
        }
        config_attribs.push(egl::NONE as EGLint);

        for (key, val) in self.options.context_attribs.iter() {
            context_attribs.push(*key as EGLint);
            context_attribs.push(*val as EGLint);
        }
        context_attribs.push(egl::NONE as EGLint);

        crate::bind_api(egl::OPENGL_ES_API).unwrap();
        crate::initialize_default(display).unwrap();
        crate::choose_config(
            display,
            config_attribs.as_ptr(),
            &mut configs,
            1,
            &mut num_config,
        )
        .unwrap();

        let win = Box::into_raw(Box::new(self.options.native_window.clone()));
        let surface =
            crate::create_window_surface(display, configs, win, std::ptr::null_mut()).unwrap();
        let context = crate::create_context(
            display,
            configs,
            std::ptr::null_mut(),
            context_attribs.as_ptr(),
        )
        .unwrap();

        crate::make_current(display, surface, surface, context).unwrap();
        Environment::new(so, display, context, surface, win)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_builder() {
        let _env = EnvironmentBuilder::defaults()
            .with_samples(2)
            .with_window_size(1920, 1080)
            .build();
    }
}
