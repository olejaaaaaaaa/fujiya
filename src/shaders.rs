use std::io::Write;
#[allow(warnings)]

use std::{fs::File, io::Read};
use ash::vk::{ShaderModule, ShaderModuleCreateInfo};

use crate::utils::read_shader_from_bytes;
use log::info;

pub struct Shader {
    pub vertex_shader: ShaderModule,
    pub fragment_shader: ShaderModule
}

#[derive(Default)]
pub struct ShaderBuilder<'n> {
    pub device: Option<&'n ash::Device>,
    pub vertex_shader_source: Option<Vec<u8>>,
    pub fragment_shader_source: Option<Vec<u8>>
}

impl<'n> ShaderBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn device(mut self, device: &'n ash::Device) -> Self {
        self.device = Some(device);
        self
    }

    fn load_from_file(path: &str) -> Vec<u8> {

        let mut file = File::open(path).unwrap();
        let mut text = Vec::new();
        file.read_to_end(&mut text).unwrap();
        file.flush().unwrap();

        assert_eq!(text.len() % 4, 0);
        assert_eq!(0x07230203, u32::from_le_bytes([text[0], text[1], text[2], text[3]]));

        info!("Загрузка шейдера: {}, размер исходника: {} байт", path, text.len());
        text
    }

    pub fn vertex_shader_source(mut self, path: &'n str) -> Self {
        let source = Self::load_from_file(path);
        self.vertex_shader_source = Some(source);
        self
    }

    pub fn fragment_shader_source(mut self, path: &'n str) -> Self {
        let source = Self::load_from_file(path);
        self.fragment_shader_source = Some(source);
        self
    }

    pub fn build(self) -> Shader {

        let binding = read_shader_from_bytes(&self.fragment_shader_source.unwrap()).unwrap();
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let fs = unsafe { self.device.unwrap().create_shader_module(&create_info, None) };

        //---------------------------------------------------

        let binding = read_shader_from_bytes(&self.vertex_shader_source.unwrap()).unwrap();
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let vs = unsafe { self.device.unwrap().create_shader_module(&create_info, None) };

        Shader { vertex_shader: vs.unwrap(), fragment_shader: fs.unwrap() }
    }
}