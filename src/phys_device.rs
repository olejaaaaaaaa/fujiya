use ash::vk::*;
use log::info;

pub struct PhysicalDevices {
    pub phys_devices: Vec<PhysicalDevice>,
    pub phys_prop: Vec<PhysicalDeviceProperties>,
    pub mem_prop: Vec<PhysicalDeviceMemoryProperties>,
    pub queue_family_prop: Vec<Vec<QueueFamilyProperties>>,
    pub features: Vec<PhysicalDeviceFeatures>
}

#[derive(Default)]
pub struct PhysicalDevicesBuilder {
    phys_devices: Vec<PhysicalDevice>,
    phys_prop: Vec<PhysicalDeviceProperties>,
    mem_prop: Vec<PhysicalDeviceMemoryProperties>,
    queue_family_prop: Vec<Vec<QueueFamilyProperties>>,
    features: Vec<PhysicalDeviceFeatures>
}

impl PhysicalDevicesBuilder {
    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn build(self, inst: &ash::Instance) -> PhysicalDevices {
        let phys_devs = unsafe { inst.enumerate_physical_devices().unwrap() };
        let mut phys_prop = vec![];
        let mut mem_prop= vec![];
        let mut queue_family_prop = vec![];
        let mut features = vec![];

        for i in &phys_devs {
            unsafe {
                let prop = inst.get_physical_device_properties(*i);
                phys_prop.push(prop);

                let mem = inst.get_physical_device_memory_properties(*i);
                mem_prop.push(mem);

                info!("Устройство: {:?}", prop.device_name_as_c_str().unwrap());

                let mut full_memory = 0;
                for i in mem.memory_heaps {
                    if i.size != 0 && i.flags.contains(MemoryHeapFlags::DEVICE_LOCAL) {
                        //println!("Размер: {:?}, флаги: {:?}", i.size / (1024 * 1024), i.flags);
                        full_memory += i.size / (1024 * 1024)
                    }
                }

                info!("Видеопамяти: {} МБ", full_memory);

                let feature = inst.get_physical_device_features(*i);
                features.push(feature);

                let queue_family = inst.get_physical_device_queue_family_properties(*i);
                queue_family_prop.push(queue_family);
            }
        }

        PhysicalDevices { phys_devices: phys_devs, phys_prop, mem_prop, queue_family_prop, features }
    }
}