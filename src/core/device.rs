use std::ffi::CStr;
use ash::vk::*;

use crate::device::core::queue::QueueFamily;

pub struct Device {
    pub raw: ash::Device
}

#[derive(Default)]
pub struct DeviceBuilder<'n> {
    extensions: Vec<*const i8>,
    features: Option<PhysicalDeviceFeatures>,
    family: Option<&'n Vec<QueueFamily>>,
    insatnce: Option<&'n ash::Instance>,
    phys_dev: Option<&'n ash::vk::PhysicalDevice>
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

    pub fn with_instance(mut self, instance: &'n ash::Instance) -> Self {
        self.insatnce = Some(instance);
        self
    }

    pub fn with_phys_dev(mut self, phys_dev: &'n ash::vk::PhysicalDevice) -> Self {
        self.phys_dev = Some(phys_dev);
        self
    }

    pub fn build(self) -> Device {

        let instance = self.insatnce.unwrap();
        let phys_dev = self.phys_dev.unwrap();

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
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&extensions)
            .enabled_features(&features);

        let device = unsafe { instance.create_device(*phys_dev, &create_info, None).unwrap() };
        Device { raw: device }
    }
}