use crate::*;
use ash::vk::{Format, PresentModeKHR};
use winit::raw_window_handle::*;
use log::*;
use winit::{dpi::PhysicalSize, raw_window_handle::HasDisplayHandle};

#[derive(Default)]
pub struct WindowManagerBuilder<'n> {
    pub winit_window: Option<winit::window::Window>,
    pub device: Option<&'n GraphicsDevice>,
    pub swapchain: Option<Swapchain>,
    pub surface: Option<Surface>,
    pub window_handle: Option<RawWindowHandle>,
    pub display_handle: Option<RawDisplayHandle>,
    pub priority_present_modes: Option<Vec<PresentModeKHR>>,
    pub priority_present_formats: Option<Vec<Format>>,
}

impl<'n> WindowManagerBuilder<'n> {
    pub fn new(window: winit::window::Window) -> Self {

        let raw_window_handle = window
            .window_handle()
            .map_err(|e| format!("Failed to get window handle: {:?}", e))
            .unwrap()
            .as_raw();

        let raw_display_handle = window
            .display_handle()
            .map_err(|e| format!("Failed to get display handle: {:?}", e))
            .unwrap()
            .as_raw();

        Self {
            winit_window: Some(window),
            window_handle: Some(raw_window_handle),
            display_handle: Some(raw_display_handle),
            ..Default::default()
        }
    }

    /// Create surface from [`crate::Instance`] or set surface
    pub fn with_surface(mut self, surface: Option<Surface>, instance: Option<&Instance>) -> Self {
        match surface {
            Some(surface) => {
                self.surface = Some(surface);
            },

            None => {

                let instance = instance.expect("Instance is Missing");
                let raw_window_handle = self.window_handle.unwrap();
                let raw_display_handle = self.display_handle.unwrap();

                let surface = SurfaceBuilder::new()
                    .with_entry(&instance.raw_entry)
                    .with_instance(&instance.raw)
                    .with_window_handle(&raw_window_handle)
                    .with_display_handle(&raw_display_handle)
                    .build();

                self.surface = Some(surface);
            }
        }

        self
    }

    /// Get a [`winit::raw_window_handle::RawWindowHandle`]
    #[inline]
    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.window_handle.unwrap()
    }

    /// Get a [`winit::raw_window_handle::RawDisplayHandle`]
    #[inline]
    pub fn get_display_handle(&self) -> RawDisplayHandle {
        self.display_handle.unwrap()
    }

    /// Setup priority present modes
    pub fn with_priority_present_modes(mut self, priority: &[PresentModeKHR]) -> Self {
        self.priority_present_modes = Some(priority.to_vec());
        self
    }

    // Setup priority present formats
    pub fn with_priority_present_formats(mut self, priority: &[Format]) -> Self {
        self.priority_present_formats = Some(priority.to_vec());
        self
    }

    pub fn with_phys_dev(mut self, device: &'n GraphicsDevice) -> Self {
        self.device = Some(device);
        self
    }

    pub fn with_swapchain(mut self, swapchain: Option<Swapchain>) -> Self {
        match swapchain {
            Some(swapchain) => {
                self.swapchain = Some(swapchain);
            },
            None => {


                let surface = self.surface.as_ref().expect("Surface is missing");
                let graphics_device = self.device.unwrap();
                let phys_dev = &graphics_device.phys_dev;
                let device = &graphics_device.device;
                let instance = &graphics_device.instance;

                let formats = surface.get_surface_formats(&phys_dev.raw);
                let caps = surface.get_surface_capabilities(&phys_dev.raw);
                let present_modes = surface.get_surface_present_modes(&phys_dev.raw);

                const DEFAULT_PRIORITY_PRESENT_MODES: &[PresentModeKHR] = &[
                    PresentModeKHR::FIFO,       // V-Sync
                    PresentModeKHR::MAILBOX     // Triple Buffering
                ];

                let present_mode = DEFAULT_PRIORITY_PRESENT_MODES
                    .iter()
                    .find(|&&mode| present_modes.contains(&mode))
                    .copied()
                    .unwrap_or_else(|| {
                        warn!("No priority mode is used, the first available one is used");
                        present_modes.first().copied().unwrap_or_else(|| {
                            panic!("No presentation modes supported!");
                        })
                    });

                const DEFAULT_PRIORITY_FORMATS: &[Format] = &[
                    Format::B8G8R8A8_SRGB,
                    Format::R8G8B8A8_SRGB,
                    Format::B8G8R8A8_UNORM,
                    Format::R8G8B8A8_UNORM,
                    Format::A8B8G8R8_SRGB_PACK32,
                ];

                let format_khr = DEFAULT_PRIORITY_FORMATS.iter()
                .find_map(|priority_fmt| {
                    formats.iter()
                        .find(|sf| sf.format == *priority_fmt)
                        .copied()
                })
                .unwrap_or_else(|| {
                    formats.first().copied().unwrap_or_else(|| {
                        panic!("No supported surface format");
                    })
                });

                let color_space = format_khr.color_space;
                let format = format_khr.format;
                let extent = caps.current_extent;

                let swapchain = SwapchainBuilder::new()
                    .with_color_space(color_space)
                    .with_format(format)
                    .with_resolution(extent)
                    .with_transform(caps.supported_transforms)
                    .with_present_mode(present_mode)
                    .with_instance(&instance.raw)
                    .with_device(&device.raw)
                    .with_surface(&surface.raw)
                    .build();

                info!("OK");

                self.swapchain = Some(swapchain);
            }
        }
        self
    }

    pub fn build(self) -> WindowManager {
        WindowManager {
            window: self.winit_window.unwrap(),
            surface: self.surface.unwrap(),
            swapchain: self.swapchain.unwrap(),
            current_present_mode: PresentModeKHR::FIFO,
            current_present_format: Format::R8G8B8A8_SRGB,
            frame_buffers: FrameBuffers { frame_buffers: vec![] },
            image_views: ImageViews { raw: vec![] }
        }
    }
}

pub struct WindowManager {
    pub window: winit::window::Window,
    pub surface: Surface,
    pub swapchain: Swapchain,
    pub current_present_mode: PresentModeKHR,
    pub current_present_format: Format,
    pub frame_buffers: FrameBuffers,
    pub image_views: ImageViews
}
