
const VS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

struct Shader {
    program : u32,
    fs_id : u32,
    vs_id : u32,
}

impl Shader {
    pub fn new(vs_src : &[u8], fs_src : &[u8]) -> Self {

        use std::ptr;
        use gl::*;

        let (program, fs_id, vs_id) = unsafe {( 
                CreateProgram(),
                CreateShader(VERTEX_SHADER),
                CreateShader(FRAGMENT_SHADER))};

        unsafe {

            ShaderSource(vs_id, 1, [vs_src.as_ptr() as *const _].as_ptr(), ptr::null());
            CompileShader(vs_id);

            ShaderSource(fs_id, 1, [fs_src.as_ptr() as *const _].as_ptr(), ptr::null());
            CompileShader(fs_id);

            AttachShader(program, vs_id);
            AttachShader(program, fs_id);
            LinkProgram(program);
            UseProgram(program);
        }

        Self {
            program, fs_id, vs_id
        }

    }
}
