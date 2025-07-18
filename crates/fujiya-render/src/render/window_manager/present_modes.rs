use ash::vk::{PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use log::warn;
use winit::window::Window;

use crate::{ Surface, WindowManagerBuilder, WithFormat};
use crate::PhysicalDevice;

pub struct WithModeAndCaps {
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub caps: SurfaceCapabilitiesKHR
}

impl WindowManagerBuilder<WithFormat> {
    pub fn with_mode<F>(self, phys_dev: &PhysicalDevice, build_fn: F) -> WindowManagerBuilder<WithModeAndCaps>
        where F: FnOnce(Vec<PresentModeKHR>) -> PresentModeKHR {

            let modes = self.state.surface.get_surface_present_modes(&phys_dev.raw);
            let caps = self.state.surface.get_surface_capabilities(&phys_dev.raw);
            let mode = build_fn(modes);

            WindowManagerBuilder { state: WithModeAndCaps {
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode,
                caps
            }}
    }

    pub fn with_default_mode(self, phys_dev: &PhysicalDevice) -> WindowManagerBuilder<WithModeAndCaps> {
        self.with_mode(phys_dev, |present_modes| {

            const DEFAULT_PRIORITY_PRESENT_MODES: &[PresentModeKHR] = &[
                PresentModeKHR::FIFO,       // V-Sync
                PresentModeKHR::MAILBOX     // Triple Buffering
            ];

            DEFAULT_PRIORITY_PRESENT_MODES
                .iter()
                .find(|&&mode| present_modes.contains(&mode))
                .copied()
                .unwrap_or_else(|| {
                    warn!("No priority mode is used, the first available one is used");
                    present_modes.first().copied().unwrap_or_else(|| {
                        panic!("No presentation modes supported!");
                    })
            })
        })
    }
}