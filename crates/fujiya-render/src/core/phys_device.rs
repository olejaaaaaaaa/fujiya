use ash::vk::*;
use log::{debug};

use crate::total_vram;

pub struct PhysicalDevice {
    pub raw: ash::vk::PhysicalDevice,
    pub phys_info: PhysicalDeviceInfo
}

#[derive(Default, Clone)]
pub struct PhysicalDeviceInfo {
    pub phys_prop: PhysicalDeviceProperties,
    pub memory_prop: PhysicalDeviceMemoryProperties,
    pub queue_family_prop: Vec<QueueFamilyProperties>,
    pub features: PhysicalDeviceFeatures,
    pub extensions: Vec<ExtensionProperties>,
    pub layers: Vec<LayerProperties>,
    pub support_surface: bool
}

#[derive(Default)]
pub struct PhysicalDeviceBuilder<'n >{
    pub instance: Option<&'n ash::Instance>,
    pub surface_load: Option<&'n ash::khr::surface::Instance>,
    pub surface: Option<&'n ash::vk::SurfaceKHR>,
    pub fn_select_phys_dev: Option<Box<dyn FnOnce(&Vec<PhysicalDeviceInfo>) -> usize>>
}

impl<'n> PhysicalDeviceBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_surface(mut self, surface: &'n ash::vk::SurfaceKHR) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn with_surface_load(mut self, surface_load: &'n ash::khr::surface::Instance) -> Self {
        self.surface_load = Some(surface_load);
        self
    }

    fn phys_device_info(&self, phys_dev: &ash::vk::PhysicalDevice, instance: &ash::Instance) -> PhysicalDeviceInfo {

        let surface_load = self.surface_load.unwrap();
        let surface = self.surface.unwrap();
        let phys_dev = *phys_dev;

        unsafe {
            let extensions = instance.enumerate_device_extension_properties(phys_dev).expect("Error get extensions");
            let layers = instance.enumerate_device_layer_properties(phys_dev).expect("Error get layer properties");
            let features = instance.get_physical_device_features(phys_dev);
            let memory_prop = instance.get_physical_device_memory_properties(phys_dev);
            let queue_prop = instance.get_physical_device_queue_family_properties(phys_dev);
            let phys_prop = instance.get_physical_device_properties(phys_dev);
            let mut support = false;

            for index in 0..queue_prop.len() {
                if surface_load.get_physical_device_surface_support(phys_dev, index as u32, *surface).unwrap_or(false) {
                    support = true;
                    break;
                }
            }

            PhysicalDeviceInfo{
                phys_prop,
                memory_prop,
                queue_family_prop: queue_prop,
                features,
                extensions,
                layers,
                support_surface: support
            }
        }
    }

    pub fn select_physical_device<F>(mut self, choose_device: F) -> Self
    where F: FnOnce(&Vec<PhysicalDeviceInfo>) -> usize + 'static
    {
        self.fn_select_phys_dev = Some(Box::new(choose_device));
        self
    }

    pub fn with_instance(mut self, instance: &'n ash::Instance) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn build(self) -> PhysicalDevice {

        let instance = self.instance.unwrap();
        let phys_devs = unsafe { instance.enumerate_physical_devices().unwrap() };
        let mut phys_infos = vec![];

        for phys_dev in &phys_devs {
            let phys_info = self.phys_device_info(phys_dev, &instance);
            if phys_info.support_surface {
                phys_infos.push(phys_info);
            }
        }

        let index = self.fn_select_phys_dev.unwrap_or(Box::new(|_| 0))(&phys_infos);
        let phys_dev = phys_devs[index];
        let phys_info = &phys_infos[index];

        let vram = total_vram(phys_info);
        debug!(
            "\nGPU NAME:        {:?}\
            \nTYPE:             {:?}\
            \nDRIVER VERSION:   {:?}\
            \nVRAM:             {:?}MB\
            \nAPI VERSION:      {:?}",
            phys_info.phys_prop.device_name_as_c_str().unwrap(),
            phys_info.phys_prop.device_type,
            phys_info.phys_prop.driver_version,
            vram / (1024 * 1024),
            phys_info.phys_prop.api_version
        );

        PhysicalDevice { raw: phys_dev, phys_info: phys_info.clone() }
    }
}