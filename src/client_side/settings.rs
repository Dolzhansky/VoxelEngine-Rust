use crate::utils::constants::{ENGINE_VERSION_MAJOR, ENGINE_VERSION_MINOR, ENGINE_VERSION_PATCH};

#[derive(Clone)]
pub struct AppSettings {
    display: DisplaySettings
}

#[derive(Clone)]
pub struct DisplaySettings {
    pub fullscreen: bool,
    pub height: u32,
    pub width: u32,
    pub title: String,
}

impl AppSettings  {
    pub fn new() -> Self {
        Self {
            display: DisplaySettings {
                fullscreen: false,
                height: 720,
                width: 1280,
                title: format!("Voxel Engine {:?}.{:?}.{:?}", ENGINE_VERSION_MAJOR, ENGINE_VERSION_MINOR, ENGINE_VERSION_PATCH),
            }
        }
    }

    pub fn get_display_settings(&mut self) -> DisplaySettings {
        let display = self.display.clone();
        return display;
    }
}