

use ash::{self, vk::{PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR}};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct Surface {
    pub surface: ash::vk::SurfaceKHR,
    pub surface_load: ash::khr::surface::Instance,
}

impl Surface {
    pub fn get_surface_formats(&self, phys_dev: &ash::vk::PhysicalDevice) -> Vec<SurfaceFormatKHR>{
        let formats = unsafe { self.surface_load.get_physical_device_surface_formats(*phys_dev, self.surface).unwrap() };
        formats
    }

    pub fn get_surface_capabilities(&self, phys_dev: &ash::vk::PhysicalDevice) -> SurfaceCapabilitiesKHR {
        let caps = unsafe { self.surface_load.get_physical_device_surface_capabilities(*phys_dev, self.surface).unwrap() };
        caps
    }

    pub fn get_surface_present_modes(&self, phys_dev: &ash::vk::PhysicalDevice) -> Vec<PresentModeKHR> {
        let present = unsafe { self.surface_load.get_physical_device_surface_present_modes(*phys_dev, self.surface).unwrap() };
        present
    }
}

#[derive(Default)]
pub struct SurfaceBuilder<'n> {
    entry: Option<&'n ash::Entry>,
    instance: Option<&'n ash::Instance>,
    window: Option<&'n winit::window::Window>,
}

impl<'n> SurfaceBuilder<'n> {
    pub fn builder() -> Self {
        SurfaceBuilder { ..Default::default() }
    }

    #[must_use]
    pub fn instance(mut self, inst: &'n ash::Instance) -> Self {
        self.instance = Some(inst);
        self
    }

    #[must_use]
    pub fn window(mut self, window: &'n winit::window::Window) -> Self {
        self.window = Some(window);
        self
    }

    #[must_use]
    pub fn entry(mut self, entry: &'n ash::Entry) -> Self {
        self.entry = Some(entry);
        self
    }

    pub fn build(self) -> Surface {
        let surface = unsafe { ash_window::create_surface(self.entry.unwrap(), self.instance.unwrap(), self.window.unwrap().display_handle().unwrap().into(), self.window.unwrap().window_handle().unwrap().into(), None).unwrap() };
        let surface_load = ash::khr::surface::Instance::new(&self.entry.unwrap(), &self.instance.unwrap());
        Surface { surface: surface, surface_load }
    }
}
