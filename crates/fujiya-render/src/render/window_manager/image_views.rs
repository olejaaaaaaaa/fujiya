use ash::vk::{Format, Image, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use winit::window::Window;

use crate::{
    Device,
    ImageViews,
    ImageViewsBuilder,
    RenderPass,
    Surface,
    Swapchain,
    WindowManagerBuilder,
    WithRenderPass,
};

pub struct WithImageViews {
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub caps: SurfaceCapabilitiesKHR,
    pub swapchain: Swapchain,
    pub render_pass: RenderPass,
    pub image_views: ImageViews
}

impl WindowManagerBuilder<WithRenderPass> {
    pub fn with_image_views<F>(self, device: &Device, build_fn: F) -> WindowManagerBuilder<WithImageViews>
        where F: FnOnce(&Device, &Format, Vec<Image>) -> ImageViews {

            let swapchain_images = self.state.swapchain.get_swapchain_images();
            let image_views = build_fn(&device, &self.state.format.format, swapchain_images);

            WindowManagerBuilder { state: WithImageViews {
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode: self.state.mode,
                caps: self.state.caps,
                swapchain: self.state.swapchain,
                render_pass: self.state.render_pass,
                image_views
            }}
    }

    pub fn with_default_image_views(self, device: &Device) -> WindowManagerBuilder<WithImageViews> {
        self.with_image_views(device, |device, format, swapchain_images| {
            ImageViewsBuilder::new()
                .with_device(&device.raw)
                .with_format(*format)
                .with_image_views(&swapchain_images)
                .build()
        })
    }
}