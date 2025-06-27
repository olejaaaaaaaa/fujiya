#[path="./core/mod.rs"]
mod core;
pub use core::app::{App, AppBuilder};
pub use core::instance::{Instance, InstanceBuilder};
pub use core::phys_device::{PhysicalDevice, PhysicalDeviceBuilder};

pub struct Device;

// Состояния билдера
pub struct NoApp;
pub struct HasApp<'n> {
    app: App<'n>,
}
pub struct HasInstance<'n> {
    app: App<'n>,
    instance: Instance,
}
pub struct Complete<'n> {
    app: App<'n>,
    instance: Instance,
    phys_dev: PhysicalDevice,
}

pub struct DeviceConfigBuilder<S> {
    state: S,
}

impl<'n> DeviceConfigBuilder<NoApp> {
    pub fn new() -> Self {
        Self {
            state: NoApp,
        }
    }

    pub fn with_app(self, app: App<'n>) -> DeviceConfigBuilder<HasApp<'n>> {
        DeviceConfigBuilder {
            state: HasApp { app },
        }
    }

    pub fn with_default_app(self) -> DeviceConfigBuilder<HasApp<'n>> {
        let app = AppBuilder::new()
            .with_app_name(c"App")
            .with_api_version(000_000_000)
            .with_engine_name(c"Fujiya")
            .with_engine_verison(24_06_2025)
            .with_api_version(ash::vk::API_VERSION_1_0)
            .build();

        self.with_app(app)
    }
}

impl<'n> DeviceConfigBuilder<HasApp<'n>> {
    pub fn with_instance<F>(self, build_fn: F) -> DeviceConfigBuilder<HasInstance<'n>>
    where
        F: FnOnce(&ash::vk::ApplicationInfo<'n>, InstanceBuilder) -> Instance,
    {
        let mut builder = InstanceBuilder::new();
        let instance = build_fn(&self.state.app.raw, builder);

        DeviceConfigBuilder {
            state: HasInstance {
                app: self.state.app,
                instance,
            },
        }
    }

    pub fn with_default_instance(self) -> DeviceConfigBuilder<HasInstance<'n>> {
        self.with_instance(|app_info, builder| {
            builder
                .add_layer(c"VK_LAYER_KHRONOS_validation")
                .with_app_info(app_info)
                .build()
        })
    }
}

