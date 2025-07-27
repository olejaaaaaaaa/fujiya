use ash::vk::{PhysicalDevice, QueueFlags};



#[derive(Default)]
pub struct QueuesFamilyBuilder<'n> {
    prop: Option<&'n Vec<ash::vk::QueueFamilyProperties>>,
    surface_load: Option<&'n ash::khr::surface::Instance>,
    surface: Option<&'n ash::vk::SurfaceKHR>,
    phys_dev: Option<&'n PhysicalDevice>
}

impl<'n> QueuesFamilyBuilder<'n> {

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

    pub fn with_queue_family_prop(mut self, queue: &'n Vec<ash::vk::QueueFamilyProperties>) -> Self {
        self.prop = Some(queue);
        self
    }

    pub fn with_phys_dev(mut self, phys_dev: &'n ash::vk::PhysicalDevice) -> Self {
        self.phys_dev = Some(&phys_dev);
        self
    }

    pub fn build(self)  -> Vec<QueueFamily> {

        let mut res = vec![];

        let families = self.prop.expect("Error not found queue family");
        let surface = self.surface.expect("Error surface");
        let surface_load = self.surface_load.expect("Err");
        let phys_dev = self.phys_dev.unwrap();

        for (index, prop) in families.iter().enumerate() {
            let support = unsafe { surface_load.get_physical_device_surface_support(*phys_dev, index as u32, *surface).unwrap_or(false) };
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
    pub raw: Vec<Vec<ash::vk::Queue>>
}

impl UniversalQueue {

    pub fn raw_graphics(&self) -> ash::vk::Queue {

        for (index, queue_family) in self.queue_family.iter().enumerate() {
            if queue_family.supports_present && queue_family.properties.queue_flags.contains(QueueFlags::GRAPHICS) {
                return self.raw[index][0]
            }
        }

        panic!("Not found Graphics Queue")
    }

    pub fn graphics_index(&self) -> u32 {

        for (index, queue_family) in self.queue_family.iter().enumerate() {
            if queue_family.supports_present && queue_family.properties.queue_flags.contains(QueueFlags::GRAPHICS) {
                return index as u32
            }
        }

        panic!("Not found Graphics Index")
    }

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

        Self { queue_family: family, raw: queue }
    }
}