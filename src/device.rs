use std::ffi::CStr;
use ash::vk::*;

use crate::queue::QueueFamily;

pub struct LogicalDevice {
    pub handle: ash::Device
}

#[derive(Default)]
pub struct DeviceBuilder<'n> {
    extensions: Vec<*const i8>,
    features: Option<PhysicalDeviceFeatures>,
    family: Option<&'n Vec<QueueFamily>>,
}

impl<'n> DeviceBuilder<'n> {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn add_extension(mut self, name: &'static CStr) -> Self {
        self.extensions.push(name.as_ptr());
        self
    }

    pub fn queue_family(mut self, family: &'n Vec<QueueFamily>) -> Self {
        self.family = Some(family);
        self
    }

    pub fn build(self, instance: &ash::Instance, phys_dev: &PhysicalDevice) -> core::result::Result<LogicalDevice, Result> {

        let family = self.family.unwrap();
        let mut priorities: Vec<Vec<f32>> = vec![];

        for i in family {
            priorities.push((1..i.properties.queue_count+1).map(|ndx| 1.0 / (ndx as f32)).collect::<Vec<f32>>());
        }

        let extensions = self.extensions;
        let features = self.features.unwrap_or(PhysicalDeviceFeatures::default());
        let mut queue_infos = vec![];

        for i in family {
            let queue_info = DeviceQueueCreateInfo::default()
                .queue_family_index(i.index)
                .queue_priorities(&priorities[i.index as usize]);

            queue_infos.push(queue_info);
        }

        let create_info = DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos.as_slice()[..1])
            .enabled_extension_names(&extensions)
            .enabled_features(&features);

        let device = unsafe { instance.create_device(*phys_dev, &create_info, None)? };
        Ok(LogicalDevice { handle: device })
    }
}