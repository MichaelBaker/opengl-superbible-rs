extern crate gl;
extern crate glfw;

use glfw::{Context, WindowHint, OpenGlProfileHint};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(3,2));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(640, 480, "This is not a window", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with( |s| window.get_proc_address(s) );

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            // Handle event
        }

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            window.swap_buffers();
        }
    }

    println!("This is not a string");
}

