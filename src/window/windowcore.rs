use glium;

use std::io::Cursor;
use glium::index::PrimitiveType;
use std::thread;
use std::time::{Duration, Instant};

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
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

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}



pub struct Window {
}

impl Window {

    pub fn new(_name : &str) -> Self {

        Self {

        }

    }


    pub fn update(&mut self) {

    }
}

pub fn test() {
    use image;
    use glium::{Surface, glutin};

 // building the display, ie. the main object
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // building a texture with "OpenGL" drawn on it
    let image = image::load(Cursor::new(&include_bytes!("resources/opengl.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let opengl_texture = glium::texture::CompressedTexture2d::new(&display, image).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(&display, 
            &[
                Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
            ]
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip,
                                               &[1 as u16, 2, 0, 3]).unwrap();

    // compiling shaders and linking them together
    let program = glium::Program::from_source(&display, r"
        #version 140
        uniform mat4 matrix;
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
            v_tex_coords = tex_coords;
        }
    ", r"
        #version 140
        uniform sampler2D tex;
        in vec2 v_tex_coords;
        out vec4 color;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    ", None).unwrap();


    let mut fullscreen = false;

    println!("Press Enter to switch fullscreen mode");

    start_loop(|| {
        // drawing a frame
        
        let mut target = display.draw();

        target.clear_color(0.0, 1.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniform! { 
                matrix: [
                    [0.5, 0.0, 0.0, 0.0],
                    [0.0, 0.5, 0.0, 0.0],
                    [0.0, 0.0, 0.5, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ],
                tex: &opengl_texture
            }, &Default::default()).unwrap();
        target.finish().unwrap();

        let mut action = Action::Continue;

        use glium::glutin::{Event,ElementState, WindowEvent, VirtualKeyCode};

        // polling and handling the events received by the window
        let mut enter_pressed = false;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, window_id } =>
                if window_id == display.gl_window().id() {
                    match event {
                        WindowEvent::Closed => action = Action::Stop,
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let ElementState::Pressed = input.state {
                                if let Some(VirtualKeyCode::Return) = input.virtual_keycode {
                                    enter_pressed = true;
                                }
                            }
                        },
                        _ => ()
                    }
                },
            _ => (),
        });

        // If enter was pressed toggle fullscreen.
        if enter_pressed {
            if fullscreen {
                let window = glutin::WindowBuilder::new();
                let context = glutin::ContextBuilder::new();
                display.rebuild(window, context, &events_loop).unwrap();
                fullscreen = false;
            } else {
                let window = glutin::WindowBuilder::new()
                    .with_fullscreen(Some(events_loop.get_primary_monitor()));
                let context = glutin::ContextBuilder::new();
                display.rebuild(window, context, &events_loop).unwrap();
                fullscreen = true;
            }
        }

        action
    });
}

