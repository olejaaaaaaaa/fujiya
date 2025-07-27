
use ash::vk::{self, DescriptorPoolSize};

pub struct DescriptorPool {
    pub raw: vk::DescriptorPool
}

#[derive(Default)]
pub struct DescriptorPoolBuilder<'n> {
    pub pool_sizes: Option<&'n [DescriptorPoolSize]>,
    pub max_sets: Option<u32>,
    pub device: Option<&'n ash::Device>,
}

impl<'n> DescriptorPoolBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_pool_sizes(mut self, pool_sizes: &'n [DescriptorPoolSize]) -> Self {
        self.pool_sizes = Some(pool_sizes);
        self
    }

    pub fn with_max_sets(mut self, max_sets: u32) -> Self {
        self.max_sets = Some(max_sets);
        self
    }

    pub fn with_device(mut self, dev: &'n ash::Device) -> Self {
        self.device = Some(dev);
        self
    }

    pub fn build(self) -> DescriptorPool {

        let device = self.device.expect("Device is missing");
        let pool_sizes = self.pool_sizes.expect("Pool sizes is missing");
        let max_sets = self.max_sets.unwrap_or(1);

        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(max_sets);

        let descriptor_pool = unsafe {
            device
                .create_descriptor_pool(&pool_info, None)
                .expect("Error create Description Pool")
        };

        DescriptorPool { raw: descriptor_pool }
    }
}