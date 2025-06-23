
use std::ffi::CStr;
use ash::{Entry, vk::*};

#[derive(Default)]
pub struct InstanceBuilder<'n> {
    app_info: Option<ApplicationInfo<'n>>,
    flags: Option<InstanceCreateFlags>,
    extensions: Vec<*const i8>,
    layers: Vec<*const i8>,
    #[allow(dead_code)]
    alloc_callback: ()
}

pub struct Instance {
    pub handle: ash::Instance,
    pub handle_entry: Entry,
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

    #[allow(warnings)]
    pub fn add_extension(mut self, name: &'static CStr) -> Self {
        self.extensions.push(name.as_ptr());
        self
    }

    pub fn add_extensions(mut self, names: Vec<&'static CStr>) -> Self {
        self.extensions.extend(names.iter().map(|name| name.as_ptr()));
        self
    }

    pub fn add_layer(mut self, name: &'static CStr) -> Self {
        self.layers.push(name.as_ptr());
        self
    }

    pub fn build(self) -> Instance {
        let entry = unsafe { Entry::load().unwrap() };

        let app_info = self.app_info.unwrap();
        let flags = self.flags.unwrap_or(InstanceCreateFlags::default());
        let layers = self.layers;
        let ext = self.extensions;

        let create_info = InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&ext)
            .enabled_layer_names(&layers)
            .flags(flags);

        let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
        Instance { handle: instance, handle_entry: entry }
    }

}

pub fn load_instance_extension_props(entry: &Entry) -> Vec<ExtensionProperties> {
    unsafe { entry.enumerate_instance_extension_properties(None).unwrap() }
}

pub fn load_instance_layer_props(entry: &Entry) -> Vec<LayerProperties> {
    unsafe { entry.enumerate_instance_layer_properties().unwrap() }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            //self.handle.destroy_instance(None);
        }
    }
}