#![cfg_attr(not(debug_assertions), windows_subsystem = "windows&")]

mod renderer;

fn main() {
    std::env::set_var("RUST_LOG", "quad_shadows");

    env_logger::init();

    let render = renderer::Render::new();

    unsafe {
        render.run();
    }
}