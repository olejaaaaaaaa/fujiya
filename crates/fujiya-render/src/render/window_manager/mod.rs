use crate::*;
use ash::vk::{Format, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use winit::{raw_window_handle::*, window::Window};

pub(crate) mod window;
pub use window::*;

pub(crate) mod surface;
pub use surface::*;

pub(crate) mod present_formats;
pub use present_formats::*;

pub(crate) mod present_modes;
pub use present_modes::*;

pub(crate) mod swapchain;
pub use swapchain::*;

pub(crate) mod render_pass;
pub use render_pass::*;

pub(crate) mod image_views;
pub use image_views::*;

pub(crate) mod frame_buffers;
pub use frame_buffers::*;

pub struct WindowManagerBuilder<S> {
    pub state: S
}

pub struct WindowManager {
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub caps: SurfaceCapabilitiesKHR,
    pub swapchain: Swapchain,
    pub render_pass: RenderPass,
    pub image_views: ImageViews,
    pub frame_buffers: FrameBuffers
}