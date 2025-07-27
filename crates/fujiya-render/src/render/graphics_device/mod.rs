
pub(crate) mod app;
pub use app::*;

pub(crate) mod instance;
pub use instance::*;

pub(crate) mod window;
pub use window::*;

pub(crate) mod phys_dev;
pub use phys_dev::*;

pub(crate) mod queue_family;
pub use queue_family::*;

pub(crate) mod device;
#[allow(unused_imports)]
pub use device::*;

use crate::{
    Instance,
    PhysicalDevice,
    Device,
    UniversalQueue
};

pub struct GraphicsDeviceBuilder<S> {
    pub state: S,
}

pub struct GraphicsDevice {
    pub instance: Instance,
    pub phys_dev: PhysicalDevice,
    pub device: Device,
    pub universal_queue: UniversalQueue,
}

impl GraphicsDevice {
    pub fn raw_device(&self) -> &ash::Device {
        &self.device.raw
    }
}
