use crate::*;

#[derive(Default)]
pub struct StandartGraphicsPipelineBuilder<'n> {
    device: Option<&'n GraphicsDevice>,
    vertex_shader: Option<Vec<u32>>,
    fragment_shader: Option<Vec<u32>>
}

impl<'n> StandartGraphicsPipelineBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_graphics_device(mut self, device: &'n GraphicsDevice) -> Self {
        self.device = Some(device);
        self
    }

    pub fn with_vertex_shader(mut self, bytes: Vec<u32>) -> Self {
        self
    }

    pub fn with_fragment_shader(mut self, bytes: Vec<u32>) -> Self {
        self
    }

    pub fn with_uniforms(mut self) -> Self {
        self
    }

    pub fn build(self) -> Self {

        let device = self.device.expect("Error get device");
        let fragment_shader = self.fragment_shader.as_ref().unwrap();
        let vertex_shader = self.vertex_shader.as_ref().unwrap();

        let shader_program = ShaderProgramBuilder::new()
            .with_device(&device.device.raw)
            .with_fragment_shader(fragment_shader.clone())
            .with_vertex_shader(vertex_shader.clone())
            .build();

        self
    }
}

pub struct StandartGraphicsPipeline {
    pipeline: RenderPipeline
}

impl StandartGraphicsPipeline {
    fn new() {

    }
}