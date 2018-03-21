use glutin::{ GlContext, EventsLoop, GlWindow, Event };
use glutin;
use gl;

pub struct Window {
    events    : EventsLoop,
    gl_window : GlWindow,
}

impl Window {

    pub fn new() -> Self {

        let events = EventsLoop::new();

        let window = glutin::WindowBuilder::new()
            .with_title("Hello, world!")
            .with_dimensions(1024, 768);

        let context = glutin::ContextBuilder::new()
            .with_vsync(true);

        let gl_window = GlWindow::new(window, context, &events).unwrap();

        Self { gl_window, events }
    }

    pub fn update(&mut self) {
        let mut running = true;

        let glw = &self.gl_window;

        self.events.poll_events(|event| {

            match event {
                Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => glw.resize(w, h),
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

