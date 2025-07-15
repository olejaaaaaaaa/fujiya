#![allow(warnings)]

use std::{
    fs::File, io::Read
};
use ash::vk::{
    ShaderModule,
    ShaderModuleCreateInfo
};

pub struct ShaderProgram {
    pub vertex_shader: ShaderModule,
    pub fragment_shader: ShaderModule
}

#[derive(Default)]
pub struct ShaderProgramBuilder<'n> {
    pub device: Option<&'n ash::Device>,
    pub vertex_shader_source: Option<Vec<u32>>,
    pub fragment_shader_source: Option<Vec<u32>>,
    pub allocation: ()
}

impl<'n> ShaderProgramBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_device(mut self, device: &'n ash::Device) -> Self {
        self.device = Some(device);
        self
    }

    pub fn with_vertex_shader(mut self, bytes: Vec<u32>) -> Self {
        self.vertex_shader_source = Some(bytes);
        self
    }

    pub fn with_fragment_shader(mut self, bytes: Vec<u32>) -> Self {
        self.fragment_shader_source = Some(bytes);
        self
    }

    pub fn build(self) -> ShaderProgram {

        let binding = self.fragment_shader_source.unwrap();
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let fs = unsafe { self.device.unwrap().create_shader_module(&create_info, None) };

        //---------------------------------------------------

        let binding = self.vertex_shader_source.unwrap();
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let vs = unsafe { self.device.unwrap().create_shader_module(&create_info, None) };

        ShaderProgram { vertex_shader: vs.unwrap(), fragment_shader: fs.unwrap() }
    }
}