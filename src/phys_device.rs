use ash::vk::*;
use log::info;


pub struct PhysicalDevice {
    pub handle: ash::vk::PhysicalDevice,
    pub phys_info: PhyscialDeviceInfo
}

#[derive(Default)]
pub struct PhysicalDevicesBuilder;

#[derive(Default, Clone)]
pub struct PhyscialDeviceInfo {
    pub phys_prop: PhysicalDeviceProperties,
    pub memory_prop: PhysicalDeviceMemoryProperties,
    pub queue_family_prop: Vec<QueueFamilyProperties>,
    pub features: PhysicalDeviceFeatures,
    pub extensions: Vec<ExtensionProperties>,
    pub layers: Vec<LayerProperties>
}

impl PhysicalDevicesBuilder {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn phys_device_info(phys_dev: &ash::vk::PhysicalDevice, instance: &ash::Instance) -> PhyscialDeviceInfo {

        let phys_dev = *phys_dev;

        unsafe {
            let extensions = instance.enumerate_device_extension_properties(phys_dev).expect("Error get extensions");
            let layers = instance.enumerate_device_layer_properties(phys_dev).expect("Error get layer properties");
            let features = instance.get_physical_device_features(phys_dev);
            let memory_prop = instance.get_physical_device_memory_properties(phys_dev);
            let queue_prop = instance.get_physical_device_queue_family_properties(phys_dev);
            let phys_prop = instance.get_physical_device_properties(phys_dev);
            PhyscialDeviceInfo{ phys_prop, memory_prop, queue_family_prop: queue_prop, features, extensions, layers }
        }
    }

    pub fn build(self, instance: &ash::Instance) -> PhysicalDevice {

        let phys_devs = unsafe { instance.enumerate_physical_devices().unwrap() };
        let mut phys_info = vec![];

        for i in &phys_devs {
            phys_info.push(Self::phys_device_info(i, &instance));
        }

        PhysicalDevice { handle: phys_devs[0], phys_info: phys_info[0].clone() }
    }
}