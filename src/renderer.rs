use glutin::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glow::{HasContext, NativeBuffer, NativeProgram, NativeVertexArray};

pub struct Render;

impl Render {
    pub fn new() -> Self {
        Self
    }

    pub unsafe fn run(&self) -> Self {
        let evloop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Hello, quad!")
            .with_inner_size(LogicalSize::new(1024.0, 768.0));

        log::info!("window created");

        // Build OpenGL context, make it current & enable v-sync
        let ctx = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window, &evloop)
            .unwrap()
            .make_current()
            .unwrap();

        log::debug!("context is now current");

        let gl = glow::Context::from_loader_function(|s| ctx.get_proc_address(s) as *const _);

        let frag = include_str!("../shaders/shader.frag");
        let vert = include_str!("../shaders/shader.vert");

        let program = Self::compile_shaders(&gl, vert, frag);

        gl.use_program(Some(program));

        let (vbo, vao) = Self::draw_quad(&gl);

        Self::set_uniform(&gl, program, "blue", 0.8);

        // Outside of event loop, doesn't clear the screen every frame
        gl.clear_color(0.3, 0.3, 0.3, 1.0);

        // Start the widnow & draw triangle
        evloop.run(move |event, _, ctrl_flow| {
            *ctrl_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    ctx.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.draw_arrays(glow::TRIANGLES, 0, 6);

                    ctx.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        ctx.resize(*physical_size);

                        let w = ctx.window().inner_size().width;
                        let h = ctx.window().inner_size().height;

                        gl.viewport(0, 0, w as i32, h as i32);
                    }
                    WindowEvent::CloseRequested => {
                        log::info!("closing window");
                        log::info!("cleaning up...");

                        gl.delete_program(program);
                        gl.delete_vertex_array(vao);
                        gl.delete_buffer(vbo);

                        log::warn!("goodbye!");

                        *ctrl_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }

    unsafe fn compile_shaders(
        gl: &glow::Context,
        vertex: &str,
        fragment: &str,
    ) -> NativeProgram {
        let program = gl.create_program().expect("Cannot create program");

        log::info!("program created");

        // Bind shader sources to their types
        let shader_sources = [
            (glow::VERTEX_SHADER, vertex),
            (glow::FRAGMENT_SHADER, fragment),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        // Iterate shader source, set types & compile
        shader_sources
            .iter()
            .for_each(|(shader_type, shader_source)| {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("cannot create shader");

                gl.shader_source(shader, &*shader_source);
                gl.compile_shader(shader);

                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }

                gl.attach_shader(program, shader);

                shaders.push(shader);
            });

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        // Detach shader objects from attached program
        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        program
    }

    unsafe fn draw_quad(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
        let quad = [
            -0.5f32, -0.5f32, 0.0f32, // a
             0.5f32, -0.5f32, 0.0f32, // b
            -0.5f32,  0.5f32, 0.0f32, // d
            -0.5f32,  0.5f32, 0.0f32, // d
             0.5f32, -0.5f32, 0.0f32, // b
             0.5f32,  0.5f32, 0.0f32, // c
        ];

        let quad_u8: &[u8] = core::slice::from_raw_parts(
            quad.as_ptr() as *const u8,
            quad.len() * core::mem::size_of::<f32>(),
        );

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, quad_u8, glow::STATIC_DRAW);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);

        (vbo, vao)
    }

    unsafe fn set_uniform(gl: &glow::Context, program: NativeProgram, name: &str, value: f32) {
        let uniform_location = gl.get_uniform_location(program, name);

        gl.uniform_1_f32(uniform_location.as_ref(), value)
    }
}

impl Default for Render {
    fn default() -> Self {
        Self::new()
    }
}