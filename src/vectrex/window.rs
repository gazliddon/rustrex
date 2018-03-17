// #![allow(single_match)]

extern crate gl;
extern crate glutin;
// use self::glutin::{GlContext};

pub struct Window {
    events_loop : glutin::EventsLoop,
    gl_window : glutin::GlWindow,
}

impl Window {

    pub fn new() -> Window {

        let events_loop = glutin::EventsLoop::new();

        let window = glutin::WindowBuilder::new()
            .with_title("Hello world!")
            .with_dimensions(1024, 768);

        let context = glutin::ContextBuilder::new();
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        Window {
            events_loop,
            gl_window,
        }
    }

    pub fn update(&mut self) {
        // use self::glutin::{VirtualKeyCode, Event, WindowEvent};

        // let ev_loop = &mut self.events_loop;
        // let win = &mut self.gl_window;

        // ev_loop.poll_events( |event| {

        //     println!("{:?}", event);


        //     match event {

        //         Event::WindowEvent { event, .. } => match event {

        //             WindowEvent::KeyboardInput {input, ..} => {
        //                 if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
        //                     panic!("quit")
        //                 }
        //             },

        //             WindowEvent::Closed => panic!("quit"),
        //             WindowEvent::Resized(w, h) => win.resize(w, h),
        //             _ => (),
        //         },
        //         _ => ()
        //     }

        //     // gl.draw_frame([0.0, 1.0, 0.0, 1.0]);

        //     let _ = win.swap_buffers();
        //     // glutin::ControlFlow::Continue
        // });
    }


}


