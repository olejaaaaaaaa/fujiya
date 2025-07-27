
use ash::{self, vk::{ComponentMapping, ComponentSwizzle, Format, Image, ImageAspectFlags, ImageSubresourceRange, ImageViewCreateInfo, ImageViewType}};

pub struct ImageViews {
    pub raw: Vec<ash::vk::ImageView>
}

#[derive(Default)]
pub struct ImageViewsBuilder<'n> {
    device: Option<&'n ash::Device>,
    images: Option<&'n Vec<Image>>,
    format: Option<Format>
}

impl<'n> ImageViewsBuilder<'n> {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_device(mut self, device: &'n ash::Device) -> Self {
        self.device = Some(device);
        self
    }

    pub fn with_image_views(mut self, images: &'n Vec<Image>) -> Self {
        self.images = Some(images);
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> ImageViews {

        let mut image_views: Vec<ash::vk::ImageView> = vec![];

        for i in self.images.unwrap() {

            let create_view_info = ImageViewCreateInfo::default()
                .view_type(ImageViewType::TYPE_2D)
                .format(self.format.unwrap())
                .components(ComponentMapping {
                    r: ComponentSwizzle::R,
                    g: ComponentSwizzle::G,
                    b: ComponentSwizzle::B,
                    a: ComponentSwizzle::A,
                })
                .subresource_range(ImageSubresourceRange {
                    aspect_mask: ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image(*i);

            let image_view = unsafe { self.device.unwrap().create_image_view(&create_view_info, None).unwrap() };
            image_views.push(image_view)
        }

        ImageViews { raw: image_views }
    }
}