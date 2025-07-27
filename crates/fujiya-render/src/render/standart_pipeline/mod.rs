use std::mem::offset_of;

use ash::vk::{self,
    AttachmentReference,
    DescriptorSetLayout,
    PrimitiveTopology
};

use crate::{
    RenderContext,
    RenderPassBuilder,
    RenderPipeline,
    RenderPipelineBuilder,
    ShaderProgramBuilder,
    SubpassBuilder
};

#[derive(Default)]
pub struct StandartPipelineBuilder<'n> {
    pub ctx: Option<&'n RenderContext>,
    pub vertex_shader: Option<Vec<u32>>,
    pub fragment_shader: Option<Vec<u32>>
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

impl Vertex {

    fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }

    fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Self, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Self, color) as u32,
            },
        ]
    }
}



impl<'n> StandartPipelineBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_graphics_device(mut self, ctx: &'n RenderContext) -> Self {
        self.ctx = Some(ctx);
        self
    }

    pub fn with_vertex_shader(mut self, bytes: Vec<u32>) -> Self {
        self.vertex_shader = Some(bytes);
        self
    }

    pub fn with_fragment_shader(mut self, bytes: Vec<u32>) -> Self {
        self.fragment_shader = Some(bytes);
        self
    }

    pub fn build(self, desc: DescriptorSetLayout) -> RenderPipeline {

        let ctx = self.ctx.unwrap();

        let shader = ShaderProgramBuilder::new()
            .with_device(&ctx.graphics_device.device.raw)
            .with_fragment_shader(self.fragment_shader.unwrap())
            .with_vertex_shader(self.vertex_shader.unwrap())
            .build();

        let subpass = SubpassBuilder::new()
            .add_color_attachment_ref(
                AttachmentReference::default()
                    .attachment(0)
                    .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            )
            .with_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .build();

        let render_pass = RenderPassBuilder::new()
            .with_device(&ctx.graphics_device.device.raw)
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
                    format: ctx.window_manager.format.format,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op: vk::AttachmentStoreOp::STORE,
                    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    ..Default::default()
                })
            .build();

        let binding_description = Vertex::get_binding_descriptions();
        let attribute_description = Vertex::get_attribute_descriptions();

        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_attribute_descriptions(&attribute_description)
            .vertex_binding_descriptions(&binding_description);

        let pipeline = RenderPipelineBuilder::new()
            .with_vertex_shader(shader.vertex_shader)
            .with_fragment_shader(shader.fragment_shader)
            .with_resolution(ctx.window_manager.caps.current_extent)
            .with_format(ctx.window_manager.format.format)
            .with_vertex_input_info(vertex_input_state_info)
            .with_input_assembly_info(
                vk::PipelineInputAssemblyStateCreateInfo::default()
                            .topology(PrimitiveTopology::TRIANGLE_LIST)
                            .primitive_restart_enable(false)
            )
            .with_render_pass(&render_pass.raw)
            .with_device(&ctx.graphics_device.device.raw)
            .build(desc);

        pipeline
    }
}

