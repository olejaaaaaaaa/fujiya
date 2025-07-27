use ash::vk::{self, CommandBuffer};

use crate::{GPUBuffer, GraphicsDevice, GraphicsDeviceBuilder, RenderPipeline, WindowManager, WindowManagerBuilder};


pub struct RenderContext {
    pub graphics_device: GraphicsDevice,
    pub window_manager: WindowManager
}

impl RenderContext {

    pub fn new(graphics_device: GraphicsDevice, window_manager: WindowManager) -> Self {
        Self {
            graphics_device,
            window_manager
        }
    }

    pub fn default(window: winit::window::Window) -> Self {

        let device = GraphicsDeviceBuilder::new()
            .with_default_app()
            .with_window(&window)
            .with_default_instance();

        let window = WindowManagerBuilder::new(window)
            .with_default_surface(&device.state.instance);

        let device = device
            .with_default_phys_dev(&window.state.surface)
            .with_default_queue_family(&window.state.surface)
            .build();

        let window = window
            .with_default_format(&device.phys_dev)
            .with_default_mode(&device.phys_dev)
            .with_default_swapchain(&device.instance, &device.device)
            .with_default_render_pass(&device.device)
            .with_default_image_views(&device.device)
            .build(&device.device);

        Self::new(device, window)
    }
}

