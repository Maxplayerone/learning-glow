extern crate gl;
extern crate glutin;

use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

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

    let vertices: [f32; 18] = [
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
    ];   
   
    //the triangle is upside down
    let vertices: [f32; 18] = [
        0.0, -0.5, 0.0, 1.0, 0.0, 0.0, 
        -0.5, 0.5, 0.0, 0.0, 1.0, 0.0,
        0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
    ];


    let program = Program::new("src/shaders/basic.vs", "src/shaders/basic.fs");
    let mut vbo = 0;
    let mut vao = 0;

    let uniformLocation;
    let uniform_vertex;

    unsafe {
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
            (6 * mem::size_of::<GLfloat>()).try_into().unwrap(),
            ptr::null(),
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,                                                   //location
            3,                                                   //amount of data
            gl::FLOAT,                                           //type of data
            gl::FALSE as GLboolean, //should the data be normalized (changed to -1 to 1) values?
            (6 * mem::size_of::<GLfloat>()).try_into().unwrap(), //how long (in bytes) is the amount of data
            mem::transmute(3 * mem::size_of::<GLfloat>()),
        );

        uniformLocation = gl::GetUniformLocation(program.id(), CString::new("u_color").unwrap().as_ptr());
        uniform_vertex = gl::GetUniformLocation(program.id(), CString::new("offset_value").unwrap().as_ptr());
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

                    program.bind();
                    gl::Uniform4f(uniformLocation, 0.7, 0.4, 0.2, 1.0);
                    gl::Uniform1f(uniform_vertex, 0.2);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
                gl_window.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
