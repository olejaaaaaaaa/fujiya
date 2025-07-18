use ash::vk::{
    ColorSpaceKHR,
    Extent2D,
    Format,
    Image,
    PresentModeKHR,
    SurfaceTransformFlagsKHR,
    SwapchainKHR
};

/// Vulkan swapchain abstraction representing a collection of presentable images
/// 
/// # Fields
/// - `swapchain`: Raw Vulkan swapchain handle
/// - `swapchain_load`: Loaded swapchain extension functions
pub struct Swapchain {
    pub swapchain: SwapchainKHR,
    pub swapchain_load: ash::khr::swapchain::Device
}

impl Swapchain {
    ///
    /// Get Current swapchain images
    ///
    pub fn get_swapchain_images(&self) -> Vec<Image> {
        let images = unsafe { self.swapchain_load.get_swapchain_images(self.swapchain).expect("Failed to get swapchain images") };
        images
    }
}

///
/// Vulkan Swapchain Builder
///
#[derive(Default)]
pub struct SwapchainBuilder<'n> {
    image_color_space: Option<ColorSpaceKHR>,
    format: Option<Format>,
    resolution: Option<Extent2D>,
    transform: Option<SurfaceTransformFlagsKHR>,
    present_mode: Option<PresentModeKHR>,
    instance: Option<&'n ash::Instance>,
    device: Option<&'n ash::Device>,
    surface: Option<&'n  ash::vk::SurfaceKHR>
}

impl<'n> SwapchainBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_resolution(mut self, res: Extent2D) -> Self {
        self.resolution = Some(res);
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn with_transform(mut self, transform: SurfaceTransformFlagsKHR) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn with_color_space(mut self, color_space: ColorSpaceKHR) -> Self {
        self.image_color_space = Some(color_space);
        self
    }

    pub fn with_present_mode(mut self, present_mode: PresentModeKHR) -> Self {
        self.present_mode = Some(present_mode);
        self
    }

    pub fn with_instance(mut self, instance: &'n ash::Instance) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn with_surface(mut self, surface: &'n ash::vk::SurfaceKHR) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn with_device(mut self, device: &'n ash::Device) -> Self {
        self.device = Some(device);
        self
    }

    /// # Panics
    /// If any required parameter is not set
    pub fn build(self) -> Swapchain {

        let surface = self.surface.expect("Missing SurfaceKHR");
        let instance = self.instance.expect("Missing Vulkan instance");
        let device = self.device.expect("Missing logical device");
        let format = self.format.expect("Missing swapchain format");
        let image_color_space = self.image_color_space.expect("Missing color space");
        let resolution = self.resolution.expect("Missing resolution");
        let transform = self.transform.expect("Missing surface transform");
        let present_mode = self.present_mode.expect("Missing present mode");

        let swapchain_create_info = ash::vk::SwapchainCreateInfoKHR::default()
            .surface(*surface)
            .min_image_count(2)
            .image_color_space(image_color_space)
            .image_format(format)
            .image_extent(resolution)
            .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .pre_transform(transform)
            .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let swapchain_load = ash::khr::swapchain::Device::new(instance, device);
        let swapchain = unsafe { swapchain_load.create_swapchain(&swapchain_create_info, None).unwrap() };

        Swapchain { swapchain, swapchain_load }
    }
}