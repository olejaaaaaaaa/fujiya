use crate::{
    App,
    AppBuilder,
    GraphicsDeviceBuilder
};

pub struct WithApp<'n> {
    pub app: App<'n>,
}

impl<'n> GraphicsDeviceBuilder<()> {
    pub fn new() -> Self {
        Self {
            state: (),
        }
    }

    pub fn with_app<F>(self, build_fn: F) -> GraphicsDeviceBuilder<WithApp<'n>>
    where F: FnOnce() -> App<'n> {

        let app = build_fn();

        GraphicsDeviceBuilder {
            state: WithApp { app },
        }
    }

    pub fn with_default_app(self) -> GraphicsDeviceBuilder<WithApp<'n>> {

        self.with_app(|| {
            AppBuilder::new()
                .with_app_name(c"App")
                .with_engine_name(c"Fujiya")
                .with_engine_version(24_06_2025)
                .with_api_version(ash::vk::API_VERSION_1_0)
                .build()
        })

    }
}