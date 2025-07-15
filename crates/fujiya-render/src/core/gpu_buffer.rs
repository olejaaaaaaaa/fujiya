use ash::vk;
use crate::{find_memorytype_index, PhysicalDeviceInfo};

pub struct GPUBuffer {
    pub buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    size: u64,
    memory_type_index: u32,
    allocation: ()
}

impl GPUBuffer {
    pub fn new(
        device: &ash::Device,
        physical_device: &PhysicalDeviceInfo,
        size: u64,
        usage: vk::BufferUsageFlags,
        memory_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self, vk::Result> {
        // Создание буфера
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&buffer_info, None)? };

        // Получение требований к памяти
        let req = unsafe { device.get_buffer_memory_requirements(buffer) };

        // Поиск подходящего типа памяти
        let memory_type_index = find_memorytype_index(
            &req,
            &physical_device.memory_prop,
            memory_flags,
        ).unwrap();

        // Выделение памяти
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(req.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.allocate_memory(&alloc_info, None)? };

        // Привязка памяти к буферу
        unsafe { device.bind_buffer_memory(buffer, memory, 0)? };

        Ok(Self {
            buffer,
            memory,
            size,
            memory_type_index,
            allocation: ()
        })
    }

    pub fn upload_data<T: Copy>(&self, device: &ash::Device, data: &[T]) {
        let data_size = (std::mem::size_of_val(data)) as u64;
        assert!(data_size <= self.size, "Data too large for buffer");

        // Отображение памяти
        let ptr = unsafe {
            device.map_memory(
                self.memory,
                0,
                data_size,
                vk::MemoryMapFlags::empty(),
            ).unwrap()
        };

        // Копирование данных
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