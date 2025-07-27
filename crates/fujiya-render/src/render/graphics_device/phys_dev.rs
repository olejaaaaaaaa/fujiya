
use ash::vk::PhysicalDeviceType;

use crate::{core::{
    Instance,  Surface,
}, PhysicalDeviceBuilder, PhysicalDeviceInfo};

use super::*;

pub struct WithPhysicalDevice {
    pub instance: Instance,
    pub phys_dev: PhysicalDevice
}

impl<'n> GraphicsDeviceBuilder<WithInstance<'n>> {

    pub fn with_phys_dev<F>(self, surface: &Surface, build_fn: F) -> GraphicsDeviceBuilder<WithPhysicalDevice>
    where F: FnOnce(&Instance, &Surface) -> PhysicalDevice {

        let phys_dev = build_fn(&self.state.instance, surface);

        GraphicsDeviceBuilder {
            state: WithPhysicalDevice {
                instance: self.state.instance,
                phys_dev
            }
        }
    }

    pub fn with_default_phys_dev(self, surface: &Surface) -> GraphicsDeviceBuilder<WithPhysicalDevice> {
        self.with_phys_dev(surface, |instance, surface| {

            const PRIORITY_GPU: &[PhysicalDeviceType] = &[
                PhysicalDeviceType::DISCRETE_GPU,
                PhysicalDeviceType::INTEGRATED_GPU,
                PhysicalDeviceType::VIRTUAL_GPU,
                PhysicalDeviceType::CPU,
                PhysicalDeviceType::OTHER
            ];

            PhysicalDeviceBuilder::new()
                .with_surface(&surface.raw)
                .with_surface_load(&surface.raw_load)
                .select_physical_device(|phys_infos: &Vec<PhysicalDeviceInfo>| {

                        for &priority_type in PRIORITY_GPU {
                            if let Some((index, _)) = phys_infos.iter().enumerate().find(|(_, info)| {
                                info.support_surface && info.phys_prop.device_type == priority_type
                            }) {
                                return index;
                            }
                        }

                        panic!("No suitable device found");
                    })
                    .with_instance(&instance.raw)
                    .build()
        })
    }
}