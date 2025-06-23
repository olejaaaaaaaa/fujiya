use crate::phys_device;







#[derive(Default)]
pub struct QueuesFamilyBuilder<'n> {
    prop: Option<&'n Vec<ash::vk::QueueFamilyProperties>>,
    surface_load: Option<&'n ash::khr::surface::Instance>,
    surface: Option<&'n ash::vk::SurfaceKHR>
}

impl<'n> QueuesFamilyBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn surface(mut self, surface: &'n ash::vk::SurfaceKHR) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn surface_load(mut self, surface_load: &'n ash::khr::surface::Instance) -> Self {
        self.surface_load = Some(surface_load);
        self
    }

    pub fn queue_family(mut self, queue: &'n Vec<ash::vk::QueueFamilyProperties>) -> Self {
        self.prop = Some(queue);
        self
    }

    pub fn build(self, phys_device: &ash::vk::PhysicalDevice)  -> Vec<QueueFamily> {

        let mut res = vec![];
        let families = self.prop.expect("Error not found queue family");
        let surface = self.surface.expect("Error surface");
        let surface_load = self.surface_load.expect("Err");

        for (index, prop) in families.iter().enumerate() {
            let support = unsafe { surface_load.get_physical_device_surface_support(*phys_device, index as u32, *surface).unwrap_or(false) };
            res.push(QueueFamily{
                index: index as u32,
                properties: *prop,
                supports_present: support
            });
        }

        res
    }
}

#[derive(Debug)]
pub struct QueueFamily {
    pub index: u32,
    pub properties: ash::vk::QueueFamilyProperties,
    pub supports_present: bool,
}

pub struct UniversalQueue {
    pub queue_family: Vec<QueueFamily>,
    pub queue: Vec<Vec<ash::vk::Queue>>
}

impl UniversalQueue {
    pub fn new(device: &ash::Device, family: Vec<QueueFamily>) -> Self {

        let mut queue = vec![];

        for i in &family {

            let mut queues = vec![];

            for j in 0..i.properties.queue_count {
                let queue = unsafe { device.get_device_queue(i.index, j) };
                queues.push(queue);
            }

            queue.push(queues)
        }

        Self { queue_family: family, queue }
    }
}