
use std::{error::Error, ffi::CStr};

use ash::vk::*;

pub struct App<'n> {
    pub app_info: ApplicationInfo<'n>
}

#[derive(Default)]
pub struct AppBuilder<'n> {
    app_info: Option<ApplicationInfo<'n>>,
    engine_name: Option<&'n CStr>,
    app_name: Option<&'n CStr>,
    engine_version: Option<u32>,
    app_version: Option<u32>,
    api_version: Option<u32>,
}

impl<'n> AppBuilder<'n> {

    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn app_name(mut self, name: &'n CStr) -> Self {
        self.app_name = Some(name);
        self
    }

    pub fn api_version(mut self, version: u32) -> Self {
        self.api_version = Some(version);
        self
    }

    pub fn engine_name(mut self, name: &'n CStr) -> Self {
        self.engine_name = Some(name);
        self
    }

    pub fn build(self) -> App<'n> {

        let api_version = self.api_version.unwrap_or(API_VERSION_1_1);
        let engine_name = self.engine_name.unwrap_or(c"None");
        let app_name = self.app_name.unwrap_or(c"None");

        let app_info = ApplicationInfo::default()
            .api_version(api_version)
            .application_name(app_name)
            .engine_name(engine_name)
            .engine_version(0)
            .application_version(0);

        App { app_info: app_info }
    }
}