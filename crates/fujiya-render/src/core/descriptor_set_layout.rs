use ash::vk;

#[derive(Default)]
pub struct DescriptorSetLayoutBuilder<'n> {
    pub bindings: Option<&'n [vk::DescriptorSetLayoutBinding<'n>]>,
    pub device: Option<&'n ash::Device>,
    pub allocation: ()
}

impl<'n> DescriptorSetLayoutBuilder<'n> {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_device(mut self, dev: &'n ash::Device) -> Self {
        self.device = Some(dev);
        self
    }

    pub fn with_bindings(mut self, bindings: &'n [vk::DescriptorSetLayoutBinding<'n>]) -> Self {
        self.bindings = Some(bindings);
        self
    }

    pub fn build(self) -> DescriptorSetLayout {
        let device = self.device.expect("Device is missing");
        let bindings = self.bindings.expect("Bingings is missing");
        let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(&bindings);

        let layout = unsafe { device.create_descriptor_set_layout(&layout_info, None).unwrap() };
        DescriptorSetLayout { raw: layout }
    }
}

#[derive(Default)]
pub struct DescriptorSetLayout {
    pub raw: vk::DescriptorSetLayout
}




/*


    let bindings = [vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX)
    ];

    let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
        .bindings(&bindings);

    let descriptor_set_layout = unsafe {
        ctx.graphics_device.device.raw
            .create_descriptor_set_layout(&layout_info, None)
            .unwrap()
    };

*/