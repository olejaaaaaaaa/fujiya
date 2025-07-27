use ash::vk::SurfaceCapabilitiesKHR;

use crate::{
    Device,
    FrameBufferBuilder,
    FrameBuffers,
    ImageViews,
    RenderPass,
    WindowManager,
    WindowManagerBuilder,
    WithImageViews
};

impl WindowManagerBuilder<WithImageViews> {
    pub fn build_with_frame_buffers<F>(self, device: &Device, build_fn: F) -> WindowManager
        where F: FnOnce(&Device, &ImageViews, &RenderPass, &SurfaceCapabilitiesKHR) -> FrameBuffers {

            let frame_buffers = build_fn(&device, &self.state.image_views, &self.state.render_pass, &self.state.caps);

            WindowManager {
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode: self.state.mode,
                caps: self.state.caps,
                swapchain: self.state.swapchain,
                render_pass: self.state.render_pass,
                image_views: self.state.image_views,
                frame_buffers
            }
    }

    pub fn build(self, device: &Device) -> WindowManager {
        self.build_with_frame_buffers(device, |device, image_views, render_pass, caps| {

                FrameBufferBuilder::new()
                    .device(&device.raw)
                    .image_views(&image_views.raw)
                    .resolution(caps.current_extent)
                    .render_pass(&render_pass.raw)
                    .build()
        })
    }
}