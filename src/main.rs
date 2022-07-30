mod util;
mod shader;
mod program;

use std::ffi::{CStr, CString};
use glfw::{Action, Context, Key};
use crate::program::Program;
use crate::shader::Shader;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(
        800,
        600,
        "ray-tracer",
        glfw::WindowMode::Windowed
    ).expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vert_shader = Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let program = Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    program.set_used();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(1.0, 0.5, 0.4, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events){
            println!("{:?}", event);

            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
