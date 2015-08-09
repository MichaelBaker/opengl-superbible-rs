extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::{Context, WindowHint, OpenGlProfileHint};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::mem::transmute;
use std::str;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(3,2));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(640, 480, "This is not a window", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with( |s| window.get_proc_address(s) );

    let program = create_program("src/vs.glsl", "src/fs.glsl");
    let mut vao = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, _) in glfw::flush_messages(&events) {
            // Handle event
        }

        unsafe {
            gl::UseProgram(program);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PointSize(10.0);
            gl::DrawArrays(gl::POINTS, 0, 1);
            window.swap_buffers();
        }
    }
}

fn check_error() {
    unsafe {
        let error = gl::GetError();
        println!("{}", error);
    }
}

fn create_program(vs_path: &str, fs_path: &str) -> u32 {
    unsafe {
        let vs_shader = load_shader(vs_path, gl::VERTEX_SHADER);
        let fs_shader = load_shader(fs_path, gl::FRAGMENT_SHADER);
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs_shader);
        gl::AttachShader(program, fs_shader);
        gl::LinkProgram(program);
        gl::DeleteShader(vs_shader);
        gl::DeleteShader(fs_shader);
        program
    }
}

fn load_shader(path: &str, shader_type: u32) -> u32 {
    unsafe {
        let source     = load_file(path);
        let shader     = gl::CreateShader(shader_type);
        let mut status = gl::FALSE as GLint;
        gl::ShaderSource(shader, 1, transmute(&source.as_bytes()), ptr::null());
        gl::CompileShader(shader);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }

        shader
    }
}

fn load_file(path: &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
