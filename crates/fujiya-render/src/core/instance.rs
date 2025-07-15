
use std::ffi::CStr;
use ash::{Entry, vk::*};
use log::{info, debug, warn};

///
/// InstanceBuilder - Contains all members for creation ash::vk::Instance
///
/// Required:
///     - ash::vk::ApplicationInfo<'_> from [`core::App`]
///
/// Default:
///     - allocation_callbacks = None
///
/// WARN: Maybe called panic if Instance is not be creation
///
#[derive(Default)]
pub struct InstanceBuilder<'n> {
    app_info: Option<ApplicationInfo<'n>>,
    flags: Option<InstanceCreateFlags>,
    extensions: Vec<*const i8>,
    layers: Vec<*const i8>,
    debug_extensions: Vec<*const i8>,
    debug_layers: Vec<*const i8>,
    allocation_callbacks: Option<AllocationCallbacks<'n>>
}

///
/// WARN: All modification raw or raw_entry then creation is unsafe and not recomended
///
pub struct Instance {
    pub raw: ash::Instance,
    pub raw_entry: Entry,
}

impl<'n> InstanceBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    #[must_use]
    pub fn with_app_info(mut self, app_info: &ApplicationInfo<'n>) -> Self {
        self.app_info = Some(*app_info);
        self
    }

    pub fn with_instance_flags(mut self, flags: InstanceCreateFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn with_extensions(mut self, names: Vec<&'static CStr>) -> Self {
        self.extensions.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    pub fn with_layers(mut self, names: Vec<&'static CStr>) -> Self {
        self.layers.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    pub fn with_debug_layers(mut self, names: Vec<&'static CStr>) -> Self {
        self.debug_layers.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    pub fn with_debug_extensions(mut self, names: Vec<&'static CStr>) -> Self {
        self.debug_extensions.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    pub fn with_allocation_callbacks(mut self, callbacks: AllocationCallbacks<'n>) -> Self {
        self.allocation_callbacks = Some(callbacks);
        self
    }

    pub fn build(self) -> Instance {

        let entry = unsafe { Entry::load().unwrap() };
        let app_info = self.app_info.expect("App info is required");
        let flags = self.flags.unwrap_or(InstanceCreateFlags::default());
        let mut layers = self.layers;
        let mut ext = self.extensions;

       cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                layers.extend(self.debug_layers);
                ext.extend(self.debug_extensions);

                for i in &layers {
                    unsafe { debug!("ENABLED LAYERS: {:?}", CStr::from_ptr(*i)); }
                }

                for i in &ext {
                    unsafe { debug!("ENABLED EXTENISONS: {:?}", CStr::from_ptr(*i)); }
                }
            }
        }

        let create_info = InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&ext)
            .enabled_layer_names(&layers)
            .flags(flags);

        let instance = unsafe { entry.create_instance(&create_info, None).expect("Error create Instance") };
        Instance { raw: instance, raw_entry: entry }
    }

}

pub fn load_instance_extension_props(entry: &Entry) -> Vec<ExtensionProperties> {
    unsafe { entry.enumerate_instance_extension_properties(None).unwrap() }
}

pub fn load_instance_layer_props(entry: &Entry) -> Vec<LayerProperties> {
    unsafe { entry.enumerate_instance_layer_properties().unwrap() }
}