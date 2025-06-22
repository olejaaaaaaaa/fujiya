use ash::{khr::swapchain, vk::*};

pub struct LogicalDevice {
    pub device: ash::Device
}

#[derive(Default)]
pub struct DeviceBuilder<'n> {
    extensions: Option<Vec<*const i8>>,
    features: Option<PhysicalDeviceFeatures>,
    inst: Option<&'n ash::Instance>,
    phys_dev: Option<&'n PhysicalDevice>,
    family_index: Option<u32>
}

impl<'n> DeviceBuilder<'n> {
    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn instance(mut self, inst: &'n ash::Instance) -> Self {
        self.inst = Some(inst);
        self
    }

    pub fn phys_dev(mut self, phys_dev: &'n PhysicalDevice) -> Self {
        self.phys_dev = Some(phys_dev);
        self
    }

    pub fn family_index(mut self, index: u32) -> Self {
        self.family_index = Some(index);
        self
    }

    pub fn build(self) -> LogicalDevice {

        let features = PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
        };

        let priorities = [1.0];

        let queue_info = DeviceQueueCreateInfo::default()
            .queue_family_index(self.family_index.unwrap())
            .queue_priorities(&priorities);

        let queue_array = [queue_info];

        let device_extension_names_raw = [
            swapchain::NAME.as_ptr(),
        ];

        let create_info = DeviceCreateInfo::default()
            .queue_create_infos(&queue_array)
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device = unsafe { self.inst.unwrap().create_device(*self.phys_dev.unwrap(), &create_info, None).unwrap() };
        LogicalDevice { device }
    }
}