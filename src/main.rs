use crate::client_side::settings::AppSettings;
use crate::client_side::window::window_manager::WindowManager;
mod engine;
mod utils;
mod client_side;
mod bridge;

fn main() {
    let mut app_settings = AppSettings::new();

    let mut window_manager = WindowManager::new(app_settings.get_display_settings()).unwrap();

    while !window_manager.should_close() {
        window_manager.glfw.poll_events();
    }

    window_manager.terminate();

}
