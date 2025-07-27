use crate::WindowManagerBuilder;
use winit::window::Window;

pub struct WithWindow {
    pub window: Window
}

impl WindowManagerBuilder<()> {
    pub fn new(window: Window) -> WindowManagerBuilder<WithWindow> {
        WindowManagerBuilder {
            state: WithWindow { window }}
    }
}


