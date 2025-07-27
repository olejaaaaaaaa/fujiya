use ash::vk::*;

pub struct RenderPass {
    pub raw: ash::vk::RenderPass
}

#[derive(Default)]
pub struct RenderPassBuilder<'n> {
    attachments: Vec<ash::vk::AttachmentDescription>,
    dependencies: Vec<ash::vk::SubpassDependency>,
    device: Option<&'n ash::Device>,
    subpass: Vec<ash::vk::SubpassDescription<'n>>
}

impl<'n> RenderPassBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_device(mut self, dev: &'n ash::Device) -> Self {
        self.device = Some(dev);
        self
    }

    pub fn add_subpass_dependency(mut self, subpass_deps: ash::vk::SubpassDependency) -> Self {
        self.dependencies.push(subpass_deps);
        self
    }

    pub fn add_subpass(mut self, subpass: ash::vk::SubpassDescription<'n>) -> Self {
        self.subpass.push(subpass);
        self
    }

    pub fn add_attachments_desc(mut self, attachment_desc: ash::vk::AttachmentDescription) -> Self {
        self.attachments.push(attachment_desc);
        self
    }

    pub fn build(self) -> RenderPass {

        let attachment_desc = self.attachments;
        let dependency = self.dependencies;
        let subpass = self.subpass;

        let create_info = RenderPassCreateInfo::default()
            .attachments(&attachment_desc)
            .subpasses(&subpass)
            .dependencies(&dependency);

        let render_pass = unsafe { self.device.unwrap().create_render_pass(&create_info, None).unwrap() };

        RenderPass { raw: render_pass }
    }
}


pub struct SubpassDescription<'n> {
    pub raw: ash::vk::SubpassDescription<'n>
}

impl<'n> SubpassDescription<'n> {
    pub fn default() -> Self {
        Self {
            raw: ash::vk::SubpassDescription::default()
        }
    }
}

pub struct AttachmentDescription {
    pub raw: ash::vk::AttachmentDescription
}

impl AttachmentDescription {
    pub fn default(format: Format) -> Self {
        Self {
            raw:
            ash::vk::AttachmentDescription {
                format: format,
                samples: SampleCountFlags::TYPE_1,
                load_op: AttachmentLoadOp::CLEAR,
                store_op: AttachmentStoreOp::STORE,
                final_layout: ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            },
        }
    }
}

pub struct SubpassDependency {
    pub raw: ash::vk::SubpassDependency
}

impl SubpassDependency {
    pub fn default() -> Self {
        Self {
            raw: ash::vk::SubpassDependency {
                src_subpass: SUBPASS_EXTERNAL,
                src_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
                dst_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                ..Default::default()
            }
        }
    }
}


#[derive(Default)]
pub struct Subpass {
    pub raw: ash::vk::SubpassDescription<'static>,
    p_color_attachment_ref: *const AttachmentReference,
    p_depth_attachment_ref: *const AttachmentReference,
    // p_resolve_attachments: *const AttachmentReference,
    // p_preserve_attachments: *const AttachmentReference
}


#[derive(Default)]
pub struct SubpassBuilder {
    color_attachment_ref: Vec<AttachmentReference>,
    depth_attachment_ref: Option<AttachmentReference>,
    bind_point: Option<PipelineBindPoint>,
    flags: Option<SubpassDescriptionFlags>
}

impl SubpassBuilder {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn add_color_attachment_ref(mut self, color_attachment_ref: AttachmentReference) -> Self {
        self.color_attachment_ref.push(color_attachment_ref);
        self
    }

    pub fn with_bind_point(mut self, bind_point: PipelineBindPoint) -> Self {
        self.bind_point = Some(bind_point);
        self
    }

    pub fn add_depth_attachment_ref(mut self, depth_attachment_ref: AttachmentReference) -> Self {
        self.depth_attachment_ref = Some(depth_attachment_ref);
        self
    }

    pub fn build(self) -> Subpass {

        let bind_point = self.bind_point.unwrap();
        let mut subpass = Subpass::default();
        subpass.raw.pipeline_bind_point = bind_point;

        if let Some(bind_point) = self.bind_point {
            subpass.raw.pipeline_bind_point = bind_point;
        }
        if let Some(flags) = self.flags {
            subpass.raw.flags = flags;
        }
        if !self.color_attachment_ref.is_empty() {
            subpass.p_color_attachment_ref = self.color_attachment_ref.as_ptr();
            subpass.raw.color_attachment_count = self.color_attachment_ref.len() as u32;
            subpass.raw.p_color_attachments = subpass.p_color_attachment_ref;
        }
        if let Some(depth) = self.depth_attachment_ref {
            subpass.p_depth_attachment_ref = Box::into_raw(Box::new(depth));
            subpass.raw.p_depth_stencil_attachment = subpass.p_depth_attachment_ref;
        }

        //     flags: todo!(),
        //     pipeline_bind_point: bind_point,
        //     input_attachment_count: todo!(),
        //     p_input_attachments: todo!(),
        //     color_attachment_count: todo!(),
        //     p_color_attachments: todo!(),
        //     p_resolve_attachments: todo!(),
        //     p_depth_stencil_attachment: todo!(),
        //     preserve_attachment_count: 0,
        //     p_preserve_attachments: todo!(),
        //     _marker: std::marker::PhantomData,
        // };

        subpass

    }

}