use ash::vk::{AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, Format, ImageLayout, PipelineBindPoint, PipelineStageFlags, RenderPassCreateInfo, SampleCountFlags, SubpassDependency, SubpassDescription, SUBPASS_EXTERNAL};

use crate::render_pass;


pub struct RenderPass {
    pub pass: ash::vk::RenderPass
}

#[derive(Default)]
pub struct RenderPassBuilder<'n> {
    attachments: Option<Vec<AttachmentDescription>>,
    format: Option<Format>,
    color_attachment_ref: Option<Vec<AttachmentReference>>,
    depth_attachment_ref: Option<Vec<AttachmentReference>>,
    dependencies: Option<SubpassDependency>,
    device: Option<&'n ash::Device>,
    subpass: Option<ash::vk::SubpassDescription<'n>>
}

impl<'n> RenderPassBuilder<'n> {

    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn device(mut self, dev: &'n ash::Device) -> Self {
        self.device = Some(dev);
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn subpass(mut self, subpass: SubpassDescription<'n>) -> Self {
        self.subpass = Some(subpass);
        self
    }

    pub fn attachments(mut self) -> Self {

        let renderpass_attachments = [

            AttachmentDescription {
                format: self.format.unwrap(),
                samples: SampleCountFlags::TYPE_1,
                load_op: AttachmentLoadOp::CLEAR,
                store_op: AttachmentStoreOp::STORE,
                final_layout: ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            },

        ];

        self.attachments = Some(renderpass_attachments.to_vec());
        self
    }

    pub fn build(self) -> RenderPass {

        let attach = self.attachments.unwrap();

        let dep = [SubpassDependency {
            src_subpass: SUBPASS_EXTERNAL,
            src_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        }];

        let color_attachment_refs = [AttachmentReference {
            attachment: 0,
            layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpass = SubpassDescription::default()
            .color_attachments(&color_attachment_refs)
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        let create_info = RenderPassCreateInfo::default()
            .attachments(&attach)
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(&dep);

        let render_pass = unsafe { self.device.unwrap().create_render_pass(&create_info, None).unwrap() };

        RenderPass { pass: render_pass }
    }
}