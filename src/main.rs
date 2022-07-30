#![feature(result_option_inspect)]

mod util;
mod error;
mod shader;
mod program;
mod resource_loader;

use gl::types::*;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use glfw::{Action, Context, Key};
use crate::program::Program;
use crate::shader::Shader;
use crate::error::Error;
use crate::resource_loader::ResourceLoader;

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

    let loader = ResourceLoader::from_relative_path(Path::new("assets")).unwrap();
    let program = Program::from_resource(&loader, "shaders/triangle").unwrap();
    program.set_used();

    /*let vert_shader = Shader::from_vert_source(
        &CString::new(include_str!("../assets/shaders/triangle.vert")).unwrap()
    )?;

    let frag_shader = Shader::from_frag_source(
        &CString::new(include_str!("../assets/shaders/triangle.frag")).unwrap()
    )?;*/

    let vertices: Vec<GLfloat> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    let mut vbo: GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut vbo); }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<GLfloat>()) as GLint,
            ptr::null()
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events){
            println!("{:?}", event);

            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height); }
                }
                _ => {}
            }
        }
    }
}
