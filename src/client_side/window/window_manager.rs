use glfw::{fail_on_errors, ffi::glfwTerminate, Context, GlfwReceiver, WindowEvent, WindowMode};
use crate::client_side::settings::DisplaySettings;

pub struct WindowManager {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub receiver: GlfwReceiver<(f64, WindowEvent)>
}

const GLFW_MAX_MAJOR_VERSION: u32 = 3;
const GLFW_MIN_MAJOR_VERSION: u32 = 3;

impl WindowManager {
    pub fn new(display_settings: DisplaySettings) -> Result<Self, String> {
        let mut glfw = glfw
        ::init(fail_on_errors!())
            .map_err(|e| format!("GLFW initialization failed: {}", e))?;

        glfw.window_hint(glfw::WindowHint::ContextVersion(GLFW_MAX_MAJOR_VERSION, GLFW_MIN_MAJOR_VERSION));
        #[cfg(target_os = "macos")]
        {
            glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
            glfw.window_hint(glfw::WindowHint::CocoaRetinaFramebuffer(false));
        }
        #[cfg(not(target_os = "macos"))]
        {
            glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Any));
        }

        glfw.window_hint(glfw::WindowHint::Resizable(true));
        //TODO: Add sampling

        let (mut window, events) = glfw
            .create_window(display_settings.width, display_settings.height, &*display_settings.title, glfw::WindowMode::Windowed)
            .ok_or_else(|| String::from("Failed to create GLFW window"))?;

        if display_settings.fullscreen {
            glfw.with_primary_monitor(|_: &mut _, m: Option<&mut glfw::Monitor>| {
                let monitor = m.unwrap();
                let mode = monitor.get_video_mode().unwrap();
                window.set_monitor(
                    glfw::WindowMode::FullScreen(&monitor),
                    0,
                    0,
                    mode.width,
                    mode.height,
                    Some(mode.refresh_rate),
                );
            });

        }

        window.set_key_polling(true);
        window.make_current();
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0, 0, display_settings.width as i32, display_settings.height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        Ok(Self { glfw, window, receiver: events })
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    pub fn width(&self) -> i32 {
        self.window.get_size().0
    }
    pub fn height(&self) -> i32 {
        self.window.get_size().1
    }

    pub fn set_should_close(&mut self, should_close: bool) {
        self.window.set_should_close(should_close)
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }
    pub fn terminate(&mut self) {
        unsafe { glfwTerminate() }
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title)
    }

    fn get_size(&self) -> (i32, i32) {
        self.window.get_size()
    }
}
