use glium;

use std::io::Cursor;
use glium::index::PrimitiveType;
use glium:: {Display , VertexBuffer, IndexBuffer, Program};
use glium::glutin;

use std::thread;
use std::time::{Duration, Instant};

pub enum Action {
    Stop,
    Continue,
}

pub fn run_loop<F>(mut callback: F) where F: FnMut() -> Action {

    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = Instant::now();

        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);

        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
        }

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
    count          : f32 ,
}


impl Window {

    pub fn update_texture(&mut self) {

        use glium::texture::{RawImage2d};
        use glium::Rect;

        const W : u32 = 600;
        const H : u32 = 297;
        const SIZE : usize = ( W * H  * 3 ) as usize;

        let mut new_data : Vec<u8> = vec![0; SIZE];

        for i in new_data.iter_mut() {
            *i = (self.count * 256.0f32)  as u8;
        }

        let ri = RawImage2d::from_raw_rgb(new_data, (W , H ));

        let rect = Rect {
            left : 0,
            bottom: 0,
            width : W,
            height: H,
        };

        self.opengl_texture.write(rect, ri);
    }

    pub fn new(_name : &str) -> Self {

        use self::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
        use glium::texture::{RawImage2d, Texture2d};

        use image;

        // building the display, ie. the main object
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new();

        let context = ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        // building a texture with "OpenGL" drawn on it
        let image = image::load(Cursor::new(&include_bytes!("resources/opengl.png")[..]),
        image::PNG).unwrap().to_rgba();

        let image_dimensions = image.dimensions();
        info!("dims {:?}", image_dimensions);
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        // let opengl_texture = CompressedTexture2d::new(&display, image).unwrap();

        let opengl_texture = Texture2d::new(&display, image).unwrap();

        // building the vertex buffer, which contains all the vertices that we will draw
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

        let vs = &include_str!("resources/standard.vs");
        let fs = &include_str!("resources/standard.fs");

        let program = glium::Program::from_source(&display, vs,fs, None).unwrap();

        Self {
            display, vertex_buffer, program,
            index_buffer, opengl_texture, events_loop, 
            count : 0.0f32,
        }
    }

    pub fn update(&mut self) -> Action {
        use glium::{Surface};

        let mut target = self.display.draw();

        self.count += 1.0f32 / 60.0f32;

        let c = self.count;

        target.clear_color(0.0, 1.0, (c * 3.0f32).cos() / 2.0f32 + 0.5f32, 1.0);

        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniform! { 
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: &self.opengl_texture

        }, &Default::default()).unwrap();

        target.finish().unwrap();

        let mut action = Action::Continue;

        use glium::glutin::{Event,ElementState, WindowEvent};

        // polling and handling the events received by the window

        self.update_texture();

        let events_loop = &mut self.events_loop;
        let display = &mut self.display;

        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, window_id } =>
                if window_id == display.gl_window().id() {
                    match event {
                        WindowEvent::Closed => action = Action::Stop,

                        WindowEvent::KeyboardInput { input, .. } => {
                            if let ElementState::Pressed = input.state {
                                use glium::glutin::VirtualKeyCode::*;

                                match input.virtual_keycode {
                                    Some(Escape) | Some(Q) => action = Action::Stop,
                                    _=> ()

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




