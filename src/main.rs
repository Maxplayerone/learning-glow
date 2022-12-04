extern crate gl;
extern crate glutin;

use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

static VS_SRC: &'static str = "
        #version 330
        layout (location=0) in vec3 position;
        layout (location=1) in vec3 a_color;

        out vec3 f_color;

        void main(){
            f_color = a_color;
            gl_Position = vec4(position, 1.0);
        }
";

static FS_SRC: &'static str = "
#version 330
out vec4 out_color;

in vec3 f_color;

void main() {
    out_color = vec4(f_color, 1.0);
}";


fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader : u32;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    let vertices : [f32; 18] = [
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vbo = 0;
    let mut vao = 0;

    let uniformLocation;

    unsafe{
        //vertex array object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        //vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (18 * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&vertices[0]), 
            gl::STATIC_DRAW);

        //shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

        //define the vertex buffer data
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (6 * mem::size_of::<GLfloat>()).try_into().unwrap(),
            ptr::null(),
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1, //location
            3, //amount of data
            gl::FLOAT, //type of data
            gl::FALSE as GLboolean, //should the data be normalized (changed to -1 to 1) values?
            (6 * mem::size_of::<GLfloat>()).try_into().unwrap(), //how long (in bytes) is the amount of data
            mem::transmute(3 * mem::size_of::<GLfloat>()) 
        );

        uniformLocation = gl::GetUniformLocation(program, CString::new("u_color").unwrap().as_ptr());
    }

    //to-do: add a uniform that will change the colour of the triangle gradually

    event_loop.run(move |event, _, control_flow| {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // Cleanup
                    unsafe{
                        gl::DeleteProgram(program);
                        gl::DeleteShader(fs);
                        gl::DeleteShader(vs);
                        gl::DeleteBuffers(1, &vbo);
                        gl::DeleteVertexArrays(1, &vao);
                    }
                    *control_flow = ControlFlow::Exit
                },
                _ => (),
            },

            Event::RedrawRequested(_) => {
                unsafe {
                    // Clear the screen to black
                    gl::ClearColor(0.4, 0.7, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    gl::UseProgram(program);
                    gl::Uniform4f(uniformLocation, 0.7, 0.4, 0.2, 1.0);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
                gl_window.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}