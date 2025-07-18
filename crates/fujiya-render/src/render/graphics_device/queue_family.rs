
use crate::{core::{
    Instance, 
}, QueueFamily, QueuesFamilyBuilder, Surface};

use super::*;

pub struct WithQueueFamily {
    pub instance: Instance,
    pub phys_dev: PhysicalDevice,
    pub queue_family: Vec<QueueFamily>
}

impl GraphicsDeviceBuilder<WithPhysicalDevice> {

pub fn with_queue_family<F>(self, surface: &Surface, build_fn: F) -> GraphicsDeviceBuilder<WithQueueFamily>
    where F: FnOnce(&Surface, &PhysicalDevice) -> Vec<QueueFamily> {

        let queue_family = build_fn(surface, &self.state.phys_dev,);

        GraphicsDeviceBuilder {
            state: WithQueueFamily {
                instance: self.state.instance,
                phys_dev: self.state.phys_dev,
                queue_family
            }
        }
    }

    pub fn with_default_queue_family(self, surface: &Surface) -> GraphicsDeviceBuilder<WithQueueFamily> {

        self.with_queue_family(surface, |surface, phys_dev| {
            QueuesFamilyBuilder::new()
                .with_queue_family_prop(&phys_dev.phys_info.queue_family_prop)
                .with_surface(&surface.raw)
                .with_surface_load(&surface.raw_load)
                .with_phys_dev(&phys_dev.raw)
                .build()
        })

    }
}