use ash::vk::*;

pub struct RenderPipeline {
    pub raw: Pipeline,
    pub raw_layout: PipelineLayout
}

#[derive(Default)]
pub struct RenderPipelineBuilder<'n> {
    device: Option<&'n ash::Device>,
    #[allow(dead_code)]
    shader_state_infos: Option<PipelineShaderStageCreateInfo<'n>>,
    input_assembly_info: Option<PipelineInputAssemblyStateCreateInfo<'n>>,
    #[allow(dead_code)]
    multisampling_info: Option<PipelineMultisampleStateCreateInfo<'n>>,
    vertex_shader: Option<ShaderModule>,
    fragment_shader: Option<ShaderModule>,
    #[allow(dead_code)]
    viewports: Option<bool>,
    #[allow(dead_code)]
    scissors: Option<bool>,
    #[allow(dead_code)]
    color_blend_attachment_state: Option<PipelineColorBlendAttachmentState>,
    #[allow(dead_code)]
    color_blending_info: Option<PipelineColorBlendStateCreateInfo<'n>>,
    vertex_input_info: Option<PipelineVertexInputStateCreateInfo<'n>>,
    resolution: Option<Extent2D>,
    format: Option<Format>,
    render_pass: Option<&'n RenderPass>
}

impl<'n> RenderPipelineBuilder<'n> {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_render_pass(mut self, pass: &'n RenderPass) -> Self {
        self.render_pass = Some(pass);
        self
    }

    pub fn with_vertex_shader(mut self, shader: ShaderModule) -> Self {
        self.vertex_shader = Some(shader);
        self
    }

    pub fn with_fragment_shader(mut self, shader: ShaderModule) -> Self {
        self.fragment_shader = Some(shader);
        self
    }

    pub fn with_vertex_input_info(mut self, input: PipelineVertexInputStateCreateInfo<'n>) -> Self {
        self.vertex_input_info = Some(input);
        self
    }

    pub fn with_resolution(mut self, res: Extent2D) -> Self {
        self.resolution = Some(res);
        self
    }

    pub fn with_input_assembly_info(mut self, input: PipelineInputAssemblyStateCreateInfo<'n>) -> Self {
        self.input_assembly_info = Some(input);
        self
    }

    // pub fn with_scissors(mut self) -> Self {
    //     self
    // }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    // pub fn with_viewports(mut self) -> Self {
    //     self
    // }

    pub fn with_device(mut self, dev: &'n ash::Device) -> Self {
        self.device = Some(dev);
        self
    }

    pub fn build(self, desc: DescriptorSetLayout) -> RenderPipeline {

        let shader_states_infos = [
            PipelineShaderStageCreateInfo::default()
                .module(self.vertex_shader.unwrap())
                .name(c"main")
                .stage(ShaderStageFlags::VERTEX),

            PipelineShaderStageCreateInfo::default()
                .module(self.fragment_shader.unwrap())
                .name(c"main")
                .stage(ShaderStageFlags::FRAGMENT),
        ];

        let vertex_input_info = self.vertex_input_info.unwrap_or(PipelineVertexInputStateCreateInfo::default());
        let input_assembly_info = self.input_assembly_info.unwrap();

        let viewports = [Viewport {
            x: 0.0,
            y: 0.0,
            width: self.resolution.unwrap().width as _,
            height: self.resolution.unwrap().height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissors = [Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: self.resolution.unwrap(),
        }];

        let viewport_info = PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer_info = PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(CullModeFlags::NONE)
            .front_face(FrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0);

        let multisampling_info = PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);

        let color_blend_attachments = [
            PipelineColorBlendAttachmentState::default()
            .color_write_mask(ColorComponentFlags::RGBA)
            .blend_enable(false)
            .src_color_blend_factor(BlendFactor::ONE)
            .dst_color_blend_factor(BlendFactor::ZERO)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ONE)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)
        ];

        let color_blending_info = PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(LogicOp::COPY)
            .attachments(&color_blend_attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let color_attachment_formats = [self.format.unwrap()];

        let mut rendering_info = PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&color_attachment_formats);

        let binding = [desc];
        let layout_info = PipelineLayoutCreateInfo::default()
            .set_layouts(&binding);

        let pipeline_layout = unsafe { self.device.unwrap().create_pipeline_layout(&layout_info, None).unwrap() };

        let pipeline_info = GraphicsPipelineCreateInfo::default()
            .stages(&shader_states_infos)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly_info)
            .viewport_state(&viewport_info)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisampling_info)
            .color_blend_state(&color_blending_info)
            .layout(pipeline_layout)
            .render_pass(*self.render_pass.unwrap())
            .push_next(&mut rendering_info);

        let pipeline = unsafe {
            self.device.unwrap()
                .create_graphics_pipelines(
                    PipelineCache::null(),
                    std::slice::from_ref(&pipeline_info),
                    None,
                )
                .map_err(|e| e.1)
        };

        RenderPipeline { raw: pipeline.unwrap()[0], raw_layout: pipeline_layout }
    }
}

