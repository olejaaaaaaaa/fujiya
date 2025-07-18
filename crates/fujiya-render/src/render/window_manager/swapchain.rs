use ash::vk::{PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use log::warn;
use winit::window::Window;

use crate::{ Device, Instance, Surface, Swapchain, SwapchainBuilder, WindowManagerBuilder, WithFormat, WithModeAndCaps };
use crate::PhysicalDevice;

pub struct WithSwapchain {
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub swapchain: Swapchain
}

impl WindowManagerBuilder<WithModeAndCaps> {
    pub fn with_swapchain<F>(self, instance: &Instance, device: &Device, build_fn: F) -> WindowManagerBuilder<WithSwapchain>
        where F: FnOnce(&Surface, &SurfaceFormatKHR, &PresentModeKHR, &SurfaceCapabilitiesKHR) -> Swapchain {

            let swapchain = build_fn(
                &self.state.surface,
                &self.state.format,
                &self.state.mode,
                &self.state.caps
            );

            WindowManagerBuilder { state: WithSwapchain {
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode: self.state.mode,
                swapchain
            }}
    }

    pub fn with_default_swapchain(self, instance: &Instance, device: &Device) -> WindowManagerBuilder<WithSwapchain> {
        self.with_swapchain(instance, device,|surface, format, mode, caps| {

            let extent = caps.current_extent;
            let transform = caps.current_transform;

            SwapchainBuilder::new()
                .with_color_space(format.color_space)
                .with_format(format.format)
                .with_resolution(extent)
                .with_transform(transform)
                .with_present_mode(*mode)
                .with_instance(&instance.raw)
                .with_device(&device.raw)
                .with_surface(&surface.raw)
                .build()
        })
    }
}