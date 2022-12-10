extern crate gl;
extern crate glutin;

use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::path::Path;

use image::EncodableLayout;

pub mod Shaders;
use Shaders::Program;

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

    let vertices: [f32; 32] = [
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0, 0.0, 0.0,
        0.5, -0.5, 0.0,     0.0, 1.0, 0.0, 1.0, 0.0,
        -0.5, 0.5, 0.0,     0.0, 0.0, 1.0, 0.0, 1.0,
        0.5, 0.5, 0.5,      1.0, 1.0, 0.0, 1.0, 1.0,
    ];

    let indices: [u32; 6] = [
        2, 0, 1,
        2, 3, 1,
    ];

    let program = Program::new("src/shaders/basic.vs", "src/shaders/basic.fs");
    let mut vbo = 0;
    let mut vao = 0;
    let mut ebo = 0;
    let mut texture = 0;

    let uniformLocation;
    let uniform_tex_loc;

    unsafe {
        //vertex array object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        //vertex buffer
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (32 * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&vertices[0]),
            gl::STATIC_DRAW,
        );

        //element buffer
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (6 * mem::size_of::<GLuint>()) as GLsizeiptr,
            mem::transmute(&indices[0]),
            gl::STATIC_DRAW,
        );

        //shader program
        program.bind();
        gl::BindFragDataLocation(program.id(), 0, CString::new("out_color").unwrap().as_ptr());


        //define the vertex buffer data
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (8 * mem::size_of::<GLfloat>()).try_into().unwrap(),
            ptr::null(),
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,                                                   //location
            3,                                                   //amount of data
            gl::FLOAT,                                           //type of data
            gl::FALSE as GLboolean, //should the data be normalized (changed to -1 to 1) values?
            (8 * mem::size_of::<GLfloat>()).try_into().unwrap(), //how long (in bytes) is the amount of data
            mem::transmute(3 * mem::size_of::<GLfloat>()),
        );

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (8 * mem::size_of::<GLfloat>()).try_into().unwrap(),
            mem::transmute(6 * mem::size_of::<GLfloat>()),
        );

        //textures
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);   
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);   
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);   
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);   


        let img = image::open(Path::new("src/assets/amogus.jpg")).unwrap().into_rgba8();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const _, 
        );
        
        uniformLocation = gl::GetUniformLocation(program.id(), CString::new("u_color").unwrap().as_ptr());
        uniform_tex_loc = gl::GetUniformLocation(program.id(), CString::new("texture0").unwrap().as_ptr());
    }

    event_loop.run(move |event, _, control_flow| {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // Cleanup
                    unsafe {
                        gl::DeleteProgram(program.id());
                        //gl::DeleteShader(fs);
                        //gl::DeleteShader(vs);
                        gl::DeleteBuffers(1, &vbo);
                        gl::DeleteVertexArrays(1, &vao);
                    }
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },

            Event::RedrawRequested(_) => {
                unsafe {
                    // Clear the screen to black
                    gl::ClearColor(0.4, 0.7, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    program.bind();
                    gl::Uniform4f(uniformLocation, 0.7, 0.4, 0.2, 1.0);
                    gl::Uniform1i(uniform_tex_loc, 0);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                }
                gl_window.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
