use ash::vk::{Extent2D, ImageView, RenderPass};


#[derive(Default)]
pub struct FrameBufferBuilder<'n> {
    resolution: Option<Extent2D>,
    render_pass: Option<&'n RenderPass>,
    image_views: Option<&'n Vec<ImageView>>,
    device: Option<&'n ash::Device>,
    #[allow(dead_code)]
    allocation: ()
}

impl<'n> FrameBufferBuilder<'n> {
    pub fn new() -> Self {
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

        let device = self.device.expect("Device is missing");
        let resolution = self.resolution.expect("Resolution is missing");
        let render_pass = self.render_pass.expect("Render Pass is missing");

        log::info!("{:?}", resolution);

        let mut frame_buffers = vec![];

        unsafe {

            for i in self.image_views.unwrap() {

                let image_view = vec![*i];

                let create_info = ash::vk::FramebufferCreateInfo::default()
                    .attachments(&image_view)
                    .attachment_count(1)
                    .width(resolution.width)
                    .height(resolution.height)
                    .layers(1)
                    .render_pass(*render_pass);

                let frame = device.create_framebuffer(&create_info, None).unwrap();
                frame_buffers.push(frame);
            }


            FrameBuffers { raw: frame_buffers }
        }

    }
}

pub struct FrameBuffers {
    pub raw: Vec<ash::vk::Framebuffer>
}