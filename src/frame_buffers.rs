use ash::vk::{Extent2D, ImageView, RenderPass};


#[derive(Default)]
pub struct FrameBufferBuilder<'n> {
    resolution: Option<Extent2D>,
    render_pass: Option<&'n RenderPass>,
    image_views: Option<&'n Vec<ImageView>>,
    device: Option<&'n ash::Device>
}

impl<'n> FrameBufferBuilder<'n> {
    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn image_views(mut self, image_views: &'n Vec<ImageView>) -> Self {
        self.image_views = Some(&image_views);
        self
    }

    pub fn resolution(mut self, res: Extent2D) -> Self {
        self.resolution = Some(res);
        self
    }

    pub fn device(mut self, device: &'n ash::Device) -> Self {
        self.device = Some(device);
        self
    }

    pub fn render_pass(mut self, pass: &'n RenderPass) -> Self {
        self.render_pass = Some(&pass);
        self
    }

    pub fn build(self) -> FrameBuffers {

        let mut frame_buffers = vec![];

        unsafe {

            for i in self.image_views.unwrap() {

                let image_view = vec![*i];

                let create_info = ash::vk::FramebufferCreateInfo::default()
                    .attachments(&image_view)
                    .attachment_count(1)
                    .width(self.resolution.unwrap().width)
                    .height(self.resolution.unwrap().height)
                    .layers(1)
                    .render_pass(*self.render_pass.unwrap());

                let frame = self.device.unwrap().create_framebuffer(&create_info, None).unwrap();
                frame_buffers.push(frame);
            }


            FrameBuffers { frame_buffers }
        }

    }
}

pub struct FrameBuffers {
    pub frame_buffers: Vec<ash::vk::Framebuffer>
}