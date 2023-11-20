extern crate gl;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // 初始化OpenGL
    gl::load_with(|s| glutin::Context::get_proc_address(s) as *const _);
    // 这里省略了OpenGL的初始化过程

    // 创建Shader并编译
    let vertex_shader_source = "#version 330 core\nlayout(location = 0) in vec3 aPos; void main() { gl_Position = vec4(aPos, 1.0); }";
    let fragment_shader_source = "#version 330 core\nout vec4 FragColor; void main() { FragColor = vec4(1.0, 0.5, 0.2, 1.0); }";

    let vertex_shader = compile_shader(gl::VERTEX_SHADER, vertex_shader_source);
    let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source);

    // 创建Program并链接Shader
    let shader_program = link_program(vertex_shader, fragment_shader);

    // 获取Shader Program的二进制数据
    let binary_data = get_program_binary(shader_program);

    // 保存二进制数据到文件
    save_binary_to_file("shader_binary.bin", &binary_data);
}

fn compile_shader(shader_type: u32, source: &str) -> u32 {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(shader, 1, &source.as_ptr() as *const _, std::ptr::null());
    gl::CompileShader(shader);

    // 检查Shader编译错误（省略错误处理）

    shader
}

fn link_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);
    gl::LinkProgram(program);

    // 检查Program链接错误（省略错误处理）

    program
}

fn get_program_binary(program: u32) -> Vec<u8> {
    let mut binary_length = 0;
    gl::GetProgramiv(program, gl::PROGRAM_BINARY_LENGTH, &mut binary_length);

    let mut binary_data = Vec::with_capacity(binary_length as usize);
    unsafe {
        binary_data.set_len(binary_length as usize);
        gl::GetProgramBinary(program, binary_length, std::ptr::null_mut(), std::ptr::null_mut(), binary_data.as_mut_ptr() as *mut _);
    }

    binary_data
}

fn save_binary_to_file(filename: &str, binary_data: &[u8]) {
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(binary_data).expect("Failed to write binary data to file");
}