use ash::vk::{self, AttachmentReference, Format, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use winit::window::Window;

use crate::{
    Device,
    RenderPass,
    RenderPassBuilder,
    SubpassBuilder,
    Surface,
    Swapchain,
    WindowManagerBuilder,
    WithSwapchain
};

pub struct WithRenderPass {
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub caps: SurfaceCapabilitiesKHR,
    pub swapchain: Swapchain,
    pub render_pass: RenderPass
}

impl WindowManagerBuilder<WithSwapchain> {

    pub fn with_render_pass<F>(self, device: &Device, build_fn: F) -> WindowManagerBuilder<WithRenderPass>
        where F: FnOnce(&Device, &Format) -> RenderPass {

            let render_pass = build_fn(device, &self.state.format.format);

            WindowManagerBuilder { state: WithRenderPass {
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode: self.state.mode,
                swapchain: self.state.swapchain,
                caps: self.state.caps,
                render_pass
            }}
    }

    pub fn with_default_render_pass(self, device: &Device) -> WindowManagerBuilder<WithRenderPass> {
        self.with_render_pass(device, |device, format| {

            let subpass = SubpassBuilder::new()
                .add_color_attachment_ref(
                    AttachmentReference::default()
                        .attachment(0)
                        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                )
                .with_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .build();

            RenderPassBuilder::new()
                .with_device(&device.raw)
                .add_subpass(subpass.raw)
                .add_subpass_dependency(
                    vk::SubpassDependency {
                        src_subpass: vk::SUBPASS_EXTERNAL,
                        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                        ..Default::default()
                    })
                .add_attachments_desc(vk::AttachmentDescription {
                        format: *format,
                        samples: vk::SampleCountFlags::TYPE_1,
                        load_op: vk::AttachmentLoadOp::CLEAR,
                        store_op: vk::AttachmentStoreOp::STORE,
                        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                        ..Default::default()
                    })
                .build()
        })
    }
}