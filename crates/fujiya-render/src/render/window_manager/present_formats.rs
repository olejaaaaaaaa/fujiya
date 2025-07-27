use ash::vk::{Format, SurfaceFormatKHR};
use winit::window::Window;

use crate::{Surface, WindowManagerBuilder, WithSurface};
use crate::PhysicalDevice;

pub struct WithFormat {
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR
}

impl WindowManagerBuilder<WithSurface> {
    pub fn with_format<F>(self, phys_dev: &PhysicalDevice, build_fn: F) -> WindowManagerBuilder<WithFormat>
        where F: FnOnce(Vec<SurfaceFormatKHR>) -> SurfaceFormatKHR {

            let formats = self.state.surface.get_surface_formats(&phys_dev.raw);
            let format = build_fn(formats);

            WindowManagerBuilder { state: WithFormat {
                window: self.state.window,
                surface: self.state.surface,
                format
            }}
    }

    pub fn with_default_format(self, phys_dev: &PhysicalDevice) -> WindowManagerBuilder<WithFormat> {
        self.with_format(phys_dev, |formats| {

                const DEFAULT_PRIORITY_FORMATS: &[Format] = &[
                    Format::B8G8R8A8_SRGB,
                    Format::R8G8B8A8_SRGB,
                    Format::B8G8R8A8_UNORM,
                    Format::R8G8B8A8_UNORM,
                    Format::A8B8G8R8_SRGB_PACK32,
                ];

                DEFAULT_PRIORITY_FORMATS.iter()
                    .find_map(|priority_fmt| {
                        formats.iter()
                            .find(|sf| sf.format == *priority_fmt)
                            .copied()
                    })
                    .unwrap_or_else(|| {
                        formats.first().copied().unwrap_or_else(|| {
                            panic!("No supported surface format");
                        })
                    })
        })
    }
}