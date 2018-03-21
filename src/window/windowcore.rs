use glutin::{ GlContext, EventsLoop, GlWindow};
// use glutin;
use gl;


pub struct Window {
    events    : EventsLoop,
    gl_window : GlWindow,
}


impl Window {

    pub fn new(name : &str) -> Self {

        use std::ffi::CStr;

        use glutin::{ContextBuilder, WindowBuilder};

        let events = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_title(name)
            .with_dimensions(1024, 768);

        let context = ContextBuilder::new()
            .with_vsync(true);

        let gl_window = GlWindow::new(window, context, &events).unwrap();

        let version = unsafe {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
            String::from_utf8(data).unwrap()
        };

        info!("OpenGl version {}", version);

        Self { gl_window, events }
    }


    pub fn update(&mut self) {
        use glutin::{ Event, WindowEvent };

        let mut running = true;

        let glw = &self.gl_window;

        self.events.poll_events(|event| {

            match event {
                Event::WindowEvent{ event, .. } => match event {
                    WindowEvent::Closed => running = false,
                    WindowEvent::Resized(w, h) => glw.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.gl_window.swap_buffers().unwrap();
    }
}

