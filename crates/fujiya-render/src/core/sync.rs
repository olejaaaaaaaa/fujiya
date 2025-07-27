use ash::vk::{self, Fence, FenceCreateFlags, Semaphore, SemaphoreCreateFlags};

#[derive(Default)]
pub struct FrameSync {
    pub image_available: Semaphore,
    pub render_finished: Semaphore,
    pub fence: Fence,
}

impl FrameSync {
    pub fn new(device: &ash::Device) -> Self {

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

        let fence_info = vk::FenceCreateInfo::default()
            .flags(FenceCreateFlags::SIGNALED);

        let fence = unsafe { device.create_fence(&fence_info, None).unwrap() };

        Self { image_available, render_finished, fence }
    }
}



