
use ash::{self, vk::{PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR}};
use winit::raw_window_handle::*;

pub struct Surface {
    pub raw: ash::vk::SurfaceKHR,
    pub raw_load: ash::khr::surface::Instance,
}

impl Surface {
    pub fn get_surface_formats(&self, phys_dev: &ash::vk::PhysicalDevice) -> Vec<SurfaceFormatKHR>{
        let formats = unsafe { self.raw_load.get_physical_device_surface_formats(*phys_dev, self.raw).unwrap() };
        formats
    }

    pub fn get_surface_capabilities(&self, phys_dev: &ash::vk::PhysicalDevice) -> SurfaceCapabilitiesKHR {
        let caps = unsafe { self.raw_load.get_physical_device_surface_capabilities(*phys_dev, self.raw).unwrap() };
        caps
    }

    pub fn get_surface_present_modes(&self, phys_dev: &ash::vk::PhysicalDevice) -> Vec<PresentModeKHR> {
        let present = unsafe { self.raw_load.get_physical_device_surface_present_modes(*phys_dev, self.raw).unwrap() };
        present
    }
}

#[derive(Default)]
pub struct SurfaceBuilder<'n> {
    entry: Option<&'n ash::Entry>,
    instance: Option<&'n ash::Instance>,
    window_handle: Option<&'n RawWindowHandle>,
    display_handle: Option<&'n RawDisplayHandle>,
    #[allow(dead_code)]
    allocation: ()
}

impl<'n> SurfaceBuilder<'n> {

    pub fn new() -> Self {
        SurfaceBuilder { ..Default::default() }
    }

    pub fn with_instance(mut self, inst: &'n ash::Instance) -> Self {
        self.instance = Some(inst);
        self
    }

    pub fn with_window_handle(mut self, handle: &'n RawWindowHandle) -> Self {
        self.window_handle = Some(handle);
        self
    }

    pub fn with_display_handle(mut self, handle: &'n RawDisplayHandle) -> Self {
        self.display_handle = Some(handle);
        self
    }

    pub fn with_entry(mut self, entry: &'n ash::Entry) -> Self {
        self.entry = Some(entry);
        self
    }

    pub fn build(self) -> Surface {
        let display_handle = self.display_handle.unwrap();
        let window_handle = self.window_handle.unwrap();
        let surface = unsafe { ash_window::create_surface(self.entry.unwrap(), self.instance.unwrap(), *display_handle, *window_handle, None).unwrap() };
        let surface_load = ash::khr::surface::Instance::new(&self.entry.unwrap(), &self.instance.unwrap());
        Surface { raw: surface, raw_load: surface_load }
    }
}
