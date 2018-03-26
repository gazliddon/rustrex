use glium;

use glium::index::PrimitiveType;
use glium:: {Display , VertexBuffer, IndexBuffer, Program};
use glium::glutin;

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Quit,
    Continue,
    Reset, 
    Pause,
}

pub fn run_loop<F>(mut callback: F) where F: FnMut() -> Action {

    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Quit => break,
            Action::Reset => (),
            Action::Continue => (),
            _ => (),
        };

        let now = Instant::now();

        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);

        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
        }

        use std::thread;
        thread::sleep(fixed_time_stamp - accumulator);
    }
}

trait GazMath {
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

struct Model {
    vertex_buffer  : VertexBuffer<Vertex>,
    index_buffer   : IndexBuffer<u16>,
}

pub struct Window {
    display        : Display,
    vertex_buffer  : VertexBuffer<Vertex>,
    index_buffer   : IndexBuffer<u16>,
    program        : Program,
    events_loop    : glutin::EventsLoop,
    opengl_texture : glium ::texture::Texture2d,
    count          : f32,
    dims           : (u32, u32),
}


impl Window {

    pub fn update_texture(&mut self, new_data : &[u8]) {

        use glium::texture::{RawImage2d};
        use glium::Rect;

        let (w, h) = self.dims;

        let ri = RawImage2d::from_raw_rgb(new_data.to_vec(), self.dims);

        let rect = Rect {
            left : 0,
            bottom: 0,
            width : w,
            height: h,
        };

        self.opengl_texture.write(rect, ri);
    }

    pub fn new(name : &str, dims : (u32, u32)) -> Self {
        let (w,h) = dims;

        use self::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
        use glium::texture::{Texture2d, UncompressedFloatFormat, MipmapsOption };
        use glium::Program;

        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new().with_title(name);
        let context = ContextBuilder::new().with_vsync(true);
        let display = Display::new(window, context, &events_loop).unwrap();

        let vertex_buffer = {
            implement_vertex!(Vertex, position, tex_coords);

            VertexBuffer::new(&display, 
                              &[
                              Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                              Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                              Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                              Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
                              ]
                             ).unwrap()
        };

        // building the index buffer
        let index_buffer = IndexBuffer::new(&display, PrimitiveType::TriangleStrip,
                                            &[1 as u16, 2, 0, 3]).unwrap();


        let opengl_texture = 
            Texture2d::empty_with_format(&display,
                                         UncompressedFloatFormat::U8U8U8,
                                         MipmapsOption::NoMipmap,
                                         w,h).unwrap();

        let program = Program::from_source(&display, 
                                           &include_str!("resources/standard.vs"),
                                           &include_str!("resources/standard.fs"),
                                           None).unwrap();

        Self {
            display, vertex_buffer, program,
            index_buffer, opengl_texture, events_loop, 
            count : 0.0f32,
            dims  : dims,
        }
    }

    pub fn update(&mut self) -> Action {
        // drae the screen
        use glium::{Surface, uniforms};
        use glium::glutin::{Event,ElementState, WindowEvent};

        self.count += 1.0f32 / 60.0f32;

        // let c = self.count * 4.0 ;
        // let r = (c * 3.0f32).cos() / 2.0f32 + 0.5f32;
        // let g = (c * -1.1f32).cos() / 2.0f32 + 0.5f32;
        // let b = (c * 1.0f32).cos() / 2.0f32 + 0.5f32;

        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.3, 1.0);

        let sampler = self.opengl_texture.sampled()
            .magnify_filter(uniforms::MagnifySamplerFilter::Nearest)
            .minify_filter(uniforms::MinifySamplerFilter::Nearest);

        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniform! { 
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex : sampler
                // tex: &self.opengl_texture
        }, &Default::default()).unwrap();

        target.finish().unwrap();

        let mut action = Action::Continue;
        let events_loop = &mut self.events_loop;
        let display = &mut self.display;

        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, window_id } =>
                if window_id == display.gl_window().id() {
                    match event {
                        WindowEvent::Closed => action = Action::Quit,

                        WindowEvent::KeyboardInput { input, .. } => {
                            if let ElementState::Pressed = input.state {
                                use glium::glutin::VirtualKeyCode::*;

                                action = match input.virtual_keycode {
                                    Some(Escape) | Some(Q) => Action::Quit,
                                    Some(R) => Action::Reset,
                                    Some(P) => Action::Pause,
                                    _=> Action::Continue,

                                };
                            }
                        },
                        _ => ()
                    }
                },
            _ => (),
        });

        action 
    }
}




