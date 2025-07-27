#![allow(warnings)]
use std::{error::Error, ffi::{c_void, CStr}, fs::File, io::{Read, Write}};
use ash::{vk::{ExtensionProperties, LayerProperties, MemoryHeapFlags}, Entry};

pub fn read_shader_from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut cursor = std::io::Cursor::new(bytes);
    Ok(ash::util::read_spv(&mut cursor)?)
}

use ash::vk;
use log::info;
use crate::core::PhysicalDeviceInfo;

pub unsafe extern "system" fn default_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    vk::FALSE
}

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}

pub fn total_vram(info: &PhysicalDeviceInfo) -> usize {
    info.memory_prop.memory_heaps
        .iter()
        .filter(|heap| heap.flags.contains(MemoryHeapFlags::DEVICE_LOCAL))
        .map(|heap| heap.size as usize)
        .sum()
}

pub fn load_spv(path: &str) -> Vec<u32> {

    let time = std::time::Instant::now();
    let mut file = File::open(path).unwrap();
    let mut text = Vec::new();
    file.read_to_end(&mut text).unwrap();
    file.flush().unwrap();

    assert_eq!(text.len() % 4, 0);
    assert_eq!(0x07230203, u32::from_le_bytes([text[0], text[1], text[2], text[3]]));

    info!("Загрузка шейдера: {}, размер исходника: {} байт, заняло времени: {} ms", path, text.len(), time.elapsed().as_millis());
    read_shader_from_bytes(&text).unwrap()
}