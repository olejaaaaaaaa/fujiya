

pub struct Queue {
    pub queue: ash::vk::Queue
}

pub struct QueueFamily {
    // index support count_queues
    pub family: Vec<(u32, bool, u32)>
}

impl QueueFamily {
    pub fn create_queue(&self, family_index: u32, device: &ash::Device) -> Queue {
        let queue = unsafe { device.get_device_queue(family_index, 0) };
        Queue { queue }
    }
}

#[derive(Default)]
pub struct QueuesFamilyBuilder<'n> {
    prop: Option<&'n Vec<ash::vk::QueueFamilyProperties>>,
    phys_dev: Option<&'n ash::vk::PhysicalDevice>,
    surface_load: Option<&'n ash::khr::surface::Instance>,
    surface: Option<&'n ash::vk::SurfaceKHR>
}

impl<'n> QueuesFamilyBuilder<'n> {

    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn phys_dev(mut self, phys_dev: &'n ash::vk::PhysicalDevice) -> Self {
        self.phys_dev = Some(phys_dev);
        self
    }

    pub fn surface(mut self, surface: &'n ash::vk::SurfaceKHR) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn surface_load(mut self, surface_load: &'n ash::khr::surface::Instance) -> Self {
        self.surface_load = Some(surface_load);
        self
    }

    pub fn queue_family_prop(mut self, queue: &'n Vec<ash::vk::QueueFamilyProperties>) -> Self {
        self.prop = Some(queue);
        self
    }

    pub fn build(self)  -> QueueFamily{

        let mut family = vec![];

        for (index, prop) in self.prop.unwrap().iter().enumerate() {
            let support = unsafe { self.surface_load.unwrap().get_physical_device_surface_support(*self.phys_dev.unwrap(), index as u32, *self.surface.unwrap()).unwrap_or(false) };
            family.push((index as u32, support, prop.queue_count))
        }

        QueueFamily { family }
    }
}