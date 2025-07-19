use glfw::{Context, GlfwReceiver, Key, Modifiers, WindowEvent};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
}

#[derive(Debug)]
pub struct Window {
    pub glfw: glfw::Glfw,
    pub deltatime: f32,

    window_handle: glfw::PWindow,
    raw_events: GlfwReceiver<(f64, WindowEvent)>,
    last_frame: f32,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            deltatime: 0.0,

            window_handle: window,
            raw_events: events,
            last_frame: 0.0,
        }
    }

    pub fn compute_deltatime(&mut self) {
        let current_frame = self.glfw.get_time() as f32;
        self.deltatime = current_frame - self.last_frame;
        self.last_frame = current_frame;
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.window_handle.get_size()
    }

    pub fn update(&mut self) -> Vec<WindowEvent> {
        let events = self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
        events
    }

    fn process_events(&mut self) -> Vec<WindowEvent> {
        glfw::flush_messages(&self.raw_events)
            .map(|(_, event)| event)
            .collect::<Vec<WindowEvent>>()
    }
}
