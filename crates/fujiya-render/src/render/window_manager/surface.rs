use crate::{core::Instance, Surface, SurfaceBuilder, WindowManagerBuilder};
use winit::{raw_window_handle::{HasDisplayHandle, HasWindowHandle}, window::Window};
use super::WithWindow;

pub struct WithSurface {
    pub window: Window,
    pub surface: Surface
}

impl WindowManagerBuilder<WithWindow> {

    pub fn with_surface<F>(self, instance: &Instance, build_fn: F) -> WindowManagerBuilder<WithSurface>
    where F: FnOnce(&Window, &Instance) -> Surface {

        let surface = build_fn(&self.state.window, instance);

        WindowManagerBuilder { state:
            WithSurface {
                window: self.state.window,
                surface
            }
        }
    }

    pub fn with_default_surface(self, instance: &Instance) -> WindowManagerBuilder<WithSurface> {
        self.with_surface(instance, |window, instance| {

            let raw_window_handle = window.window_handle().expect("Err").as_raw();
            let raw_display_handle = window.display_handle().expect("Err").as_raw();

            SurfaceBuilder::new()
                .with_entry(&instance.raw_entry)
                .with_instance(&instance.raw)
                .with_window_handle(&raw_window_handle)
                .with_display_handle(&raw_display_handle)
                .build()
        })
    }
}