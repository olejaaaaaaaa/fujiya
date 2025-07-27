use ash::vk::{self, PhysicalDeviceMemoryProperties};
use log::warn;
use crate::{find_memorytype_index};

///
/// Wraper around [`ash::vk::Buffer`] for simple use
/// # Panic
/// if size == 0
///
/// # Example:
///
/// ```
/// fn main() {
///     let uniform_buffer = GPUBuffer::new(
///         &device.raw,
///         &memory_prop,
///         buffer_size,
///         vk::BufferUsageFlags::UNIFORM_BUFFER,
///         vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
///     ).unwrap();
///
///     uniform_buffer.upload_data(&ctx.graphics_device.device.raw, &[data]);
/// }
/// ```
///
pub struct GPUBuffer {
    pub raw: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: u64,
}

impl GPUBuffer {

    /// Create [`GPUBuffer`]
    pub fn new(
        device: &ash::Device,
        memory_prop: &PhysicalDeviceMemoryProperties,
        size: u64,
        usage: vk::BufferUsageFlags,
        memory_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self, vk::Result> {

        // Buffer with size 0? WTF?
        assert_ne!(size, 0);

        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&buffer_info, None)? };
        let req = unsafe { device.get_buffer_memory_requirements(buffer) };

        let memory_type_index = find_memorytype_index(
            &req,
            memory_prop,
            memory_flags,
        ).unwrap();

        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(req.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.allocate_memory(&alloc_info, None)? };
        unsafe { device.bind_buffer_memory(buffer, memory, 0)? };

        Ok(Self {
            raw: buffer,
            memory,
            size,
        })
    }

    /// Upload data into GPU Memory
    pub fn upload_data<T: Copy>(&self, device: &ash::Device, data: &[T]) {

        let data_size = (std::mem::size_of_val(data)) as u64;
        assert!(data_size <= self.size, "Data too large for buffer");

        let ptr = unsafe {
            device.map_memory(
                self.memory,
                0,
                data_size,
                vk::MemoryMapFlags::empty(),
            ).expect("Error map memory")
        };

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                ptr as *mut u8,
                data_size as usize,
            );
        }

        unsafe { device.unmap_memory(self.memory) };
    }

}


impl Drop for GPUBuffer {
    fn drop(&mut self) {
        if self.size != 0 {
            warn!("Memory Leak");
        }
    }
}