
use std::ffi::CStr;
use ash::vk::*;

///
/// App contains ash::vk::ApplicationInfo<'_>
///
/// how to use:
/// ```
/// fn main() {
///     let app = AppBuilder::new()
///         .with_api_version(ash::vk::API_VERSION_1_0)
///         .build()
/// }
/// ```
pub struct App<'n> {
    pub raw: ApplicationInfo<'n>
}

///
/// Default values:
///     - api_version = VK_API_VERSION_1_0
///     - app_name = c"None"
///     - app_version = 0
///     - engine_name = c"Fujiya"
///     - engine_version = 0
///
/// example:
/// ```
/// fn main() {
///     let app = AppBuilder::new()
///         .with_api_version(ash::vk::API_VERSION_1_0)
///         .with_app_name(c"Mario")
///         .build()
/// }
/// ```
///
///
#[derive(Default)]
pub struct AppBuilder<'n> {
    engine_name: Option<&'n CStr>,
    app_name: Option<&'n CStr>,
    engine_version: Option<u32>,
    app_version: Option<u32>,
    api_version: Option<u32>,
}

impl<'n> AppBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_app_name(mut self, name: &'n CStr) -> Self {
        self.app_name = Some(name);
        self
    }

    pub fn with_api_version(mut self, version: u32) -> Self {
        self.api_version = Some(version);
        self
    }

    pub fn with_engine_version(mut self, version: u32) -> Self {
        self.engine_version = Some(version);
        self
    }

    pub fn with_app_version(mut self, version: u32) -> Self {
        self.app_version = Some(version);
        self
    }

    pub fn with_engine_name(mut self, name: &'n CStr) -> Self {
        self.engine_name = Some(name);
        self
    }

    pub fn build(self) -> App<'n> {

        let engine_name = self.engine_name.unwrap_or(c"Fujiya");
        let engine_version = self.api_version.unwrap_or(0);
        let app_name = self.app_name.unwrap_or(c"None");
        let app_version = self.app_version.unwrap_or(0);
        let api_version = self.api_version.unwrap_or(API_VERSION_1_0);

        let app_info = ApplicationInfo::default()
            .api_version(api_version)
            .application_name(app_name)
            .engine_name(engine_name)
            .engine_version(engine_version)
            .application_version(app_version);

        App { raw: app_info }
    }
}