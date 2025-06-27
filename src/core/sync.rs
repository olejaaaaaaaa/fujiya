
pub struct Sync {

}

#[derive(Default)]
pub struct SyncBuilder {

}

impl SyncBuilder {
    pub fn builder() {

    }

    pub fn build() {

    }
}

use ash::vk::{self, SemaphoreCreateFlags};

pub fn create_semaphores(device: &ash::Device) -> (vk::Semaphore, vk::Semaphore) {
    let image_available = {
        let semaphore_info = vk::SemaphoreCreateInfo::default()
            .flags(SemaphoreCreateFlags::default());
        unsafe { device.create_semaphore(&semaphore_info, None).unwrap() }
    };
    let render_finished = {
        let semaphore_info = vk::SemaphoreCreateInfo::default()
            .flags(SemaphoreCreateFlags::default());
        
        unsafe { device.create_semaphore(&semaphore_info, None).unwrap() }
    };
    (image_available, render_finished)
}