use ash::vk::{ColorSpaceKHR, Extent2D, Format, Image, PresentModeKHR, SurfaceKHR, SurfaceTransformFlagsKHR, SwapchainKHR};

pub struct Swapchain {
    pub swapchain: SwapchainKHR,
    pub swapchain_load: ash::khr::swapchain::Device
}

impl Swapchain {
    pub fn get_swapchain_images(&self) -> Vec<Image> {
        let images = unsafe { self.swapchain_load.get_swapchain_images(self.swapchain).unwrap() };
        images
    }
}

#[derive(Default)]
pub struct SwapchainBuilder<'n> {
    surface: Option<&'n SurfaceKHR>,
    inst: Option<&'n ash::Instance>,
    device: Option<&'n ash::Device>,
    image_color_space: Option<ColorSpaceKHR>,
    format: Option<Format>,
    resolution: Option<Extent2D>,
    transform: Option<SurfaceTransformFlagsKHR>,
    present_mode: Option<PresentModeKHR>,
}

impl<'n> SwapchainBuilder<'n> {

    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn instance(mut self, inst: &'n ash::Instance) -> Self {
        self.inst = Some(inst);
        self
    }

    pub fn device(mut self, dev: &'n ash::Device) -> Self {
        self.device = Some(dev);
        self
    }

    pub fn surface(mut self, surface: &'n SurfaceKHR) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn resolution(mut self, res: Extent2D) -> Self {
        self.resolution = Some(res);
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn transform(mut self, transform: SurfaceTransformFlagsKHR) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn color_space(mut self, color_space: ColorSpaceKHR) -> Self {
        self.image_color_space = Some(color_space);
        self
    }

    pub fn present_mode(mut self, present_mode: PresentModeKHR) -> Self {
        self.present_mode = Some(present_mode);
        self
    }

    pub fn build(self) -> Swapchain {

        let swapchain_create_info = ash::vk::SwapchainCreateInfoKHR::default()
            .surface(*self.surface.unwrap())
            .min_image_count(2)
            .image_color_space(self.image_color_space.unwrap())
            .image_format(self.format.unwrap())
            .image_extent(self.resolution.unwrap())
            .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .pre_transform(self.transform.unwrap())
            .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(self.present_mode.unwrap())
            .clipped(true)
            .image_array_layers(1);

        let swapchain_load = ash::khr::swapchain::Device::new(&self.inst.unwrap(), &self.device.unwrap());
        let swapchain = unsafe { swapchain_load.create_swapchain(&swapchain_create_info, None).unwrap() };

        Swapchain { swapchain, swapchain_load }
    }
}