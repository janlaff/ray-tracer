extern crate glfw;

use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, _) = glfw.create_window(
        800,
        600,
        "ray-tracer",
        glfw::WindowMode::Windowed
    ).expect("Failed to create window");

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
    }
}
