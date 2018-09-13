
extern crate gl;

use std::ffi::CString;
use std::ffi::CStr;
use std::iter;
use std::mem;
use std::os::raw::{c_void, c_char};
use std::ptr;

use shaders::ShaderType;

use na::{self, Matrix4};

use Program;
use Shader;
use AbstractContext;
use NativeBuffer;
use Context;

pub type GLShader = u32;
pub type GLProgram = u32;
pub type UniformLocation = i32;
pub type GLEnum = u32;
pub type GLsizeiptr = gl::types::GLsizeiptr;
pub type GLintptr = gl::types::GLintptr;
pub type GLBuffer = u32;
pub type GLVertexArray = u32;
pub type GLUint = u32;

lazy_static! {
    static ref CONTEXT: Context = GLContext::new();
}

pub struct GLContext {}

impl GLContext {
    fn new() -> Self {
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(callaback, ptr::null());
        }
        GLContext {}
    }

}

extern "system" fn callaback(source: GLEnum, type_: GLEnum, id: GLUint, severity: GLEnum, _length: i32, message: *const c_char, _user_param: *mut c_void) {
    unsafe {
        let m = CStr::from_ptr(message);
        println!("source: {:?}, type: {:?}, id: {:?}, severity: {:?}, message: {:#?}", source, type_, id, severity, m);

        if type_ == gl::DEBUG_TYPE_ERROR {
            panic!("GL ERROR");
        }
    }
}

impl AbstractContext for GLContext {
    const VERTEX_SHADER: u32 = gl::VERTEX_SHADER;
    const FRAGMENT_SHADER: u32 = gl::FRAGMENT_SHADER;
    const FLOAT: u32 = gl::FLOAT;
    const COLOR_BUFFER_BIT: u32 = gl::COLOR_BUFFER_BIT;
    const ARRAY_BUFFER: u32 = gl::ARRAY_BUFFER;
    const ELEMENT_ARRAY_BUFFER: u32 = gl::ELEMENT_ARRAY_BUFFER;
    const STATIC_DRAW: u32 = gl::STATIC_DRAW;
    const DYNAMIC_DRAW: u32 = gl::DYNAMIC_DRAW;
    const COMPILE_STATUS: u32 = gl::COMPILE_STATUS;
    const POINTS: u32 = gl::POINTS;
    const LINE_STRIP: u32 = gl::LINE_STRIP;
    const LINE_LOOP: u32 = gl::LINE_LOOP;
    const LINES: u32 = gl::LINES;
    const TRIANGLE_STRIP: u32 = gl::TRIANGLE_STRIP;
    const TRIANGLE_FAN: u32 = gl::TRIANGLE_FAN;
    const TRIANGLES: u32 = gl::TRIANGLES;
    const UNSIGNED_SHORT: u32 = gl::UNSIGNED_SHORT;

    fn get_context() -> &'static Context {
        &CONTEXT
    }
    
    fn create_shader(&self, type_: ShaderType) -> Option<Shader> {
        unsafe {
            match type_ {
                ShaderType::Vertex => Some(gl::CreateShader(Self::VERTEX_SHADER)),
                ShaderType::Fragment => Some(gl::CreateShader(Self::FRAGMENT_SHADER)),
            }
        }
    }

    fn shader_source(&self, shader: &Shader, source: &str) {
        unsafe {
            let src = CString::new(source).unwrap();
            gl::ShaderSource(*shader, 1, &src.as_ptr(), ptr::null());
        }
    }

    fn compile_shader(&self, shader: &Shader) {
        unsafe {
            gl::CompileShader(*shader);
        }
    }

    fn delete_shader(&self, shader: &Shader) {
        unsafe {
            gl::DeleteShader(*shader);
        }
    }

    fn get_shader_parameter(&self, shader: &Shader, pname: GLEnum) -> Option<i32> {
        let mut result = 0;
        unsafe {
            gl::GetShaderiv(*shader, pname, &mut result);
        }
        Some(result)
    }

    fn get_shader_info_log(&self, shader: &Shader) -> Option<String> {
        let info_length = self
            .get_shader_parameter(shader, gl::INFO_LOG_LENGTH)
            .unwrap();
        if info_length > 0 {
            let mut written_length = 0;
            let buffer: String = iter::repeat(' ').take(info_length as usize).collect();

            let buffer_string = CString::new(buffer.as_bytes()).unwrap();
            unsafe {
                gl::GetShaderInfoLog(
                    *shader,
                    info_length,
                    &mut written_length,
                    buffer_string.as_ptr() as *mut i8,
                )
            };
            let bytes = buffer_string.as_bytes();
            let bytes = &bytes[..bytes.len() - 1];
            String::from_utf8(bytes.to_vec()).ok()
        } else {
            None
        }
    }

    fn create_program(&self) -> Option<Program> {
        unsafe { Some(gl::CreateProgram()) }
    }

    fn attach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            gl::AttachShader(*program, *shader);
        }
    }

    fn link_program(&self, program: &Program) {
        unsafe {
            gl::LinkProgram(*program);
        }
    }

    fn use_program(&self, program: &Program) {
        unsafe {
            gl::UseProgram(*program);
        }
    }

    fn delete_program(&self, program: &Program) {
        unsafe {
            gl::DeleteProgram(*program);
        }
    }

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    fn clear(&self, mask: u32) {
        unsafe {
            gl::Clear(mask);
        }
    }

    fn create_buffer(&self) -> Option<NativeBuffer> {
        let mut buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
        }
        Some(buffer)
    }

    fn bind_buffer(&self, target: GLEnum, buffer: &NativeBuffer) {
        unsafe {
            gl::BindBuffer(target, *buffer);
        }
    }

    fn buffer_data<T>(&self, target: GLEnum, data: &[T], usage: GLEnum) {
        unsafe {
            gl::BufferData(
                target,
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                mem::transmute(&data[0]),
                usage,
            );  
        }
    }

    fn delete_buffer(&self, buffer: &NativeBuffer) {
        unsafe {
            gl::DeleteBuffers(1, buffer);
        }
    }

    fn get_attrib_location(&self, program: &Program, name: &str) -> GLUint {
        unsafe {
            let src = CString::new(name).unwrap();
            gl::GetAttribLocation(*program, src.as_ptr()) as GLUint
        }
    }

    fn vertex_attrib_pointer(
        &self,
        pointer: &GLUint,
        size: i32,
        type_: GLEnum,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        unsafe {
            gl::VertexAttribPointer(
                *pointer,
                size,
                type_,
                normalized as u8,
                stride * mem::size_of::<f32>() as gl::types::GLsizei,
                (offset * mem::size_of::<f32>() as i32) as *const () as *const _,
            ); // black magic
        }
    }

    fn enable_vertex_attrib_array(&self, pointer: &GLUint) {
        unsafe {
            gl::EnableVertexAttribArray(*pointer);
        }
    }
    
    fn disable_vertex_attrib_array(&self, pointer: &GLUint) {
        unsafe {
            gl::DisableVertexAttribArray(*pointer);
        }
    }
    
    fn bind_attrib_location(&self, program: &Program, index: GLUint, name: &str) {
        unsafe {
            let src = CString::new(name).unwrap();
            gl::BindAttribLocation(*program, index, src.as_ptr());
        }
    }

    fn get_uniform_location(&self, program: &Program, name: &str) -> UniformLocation {
        unsafe {
            let src = CString::new(name).unwrap();
            gl::GetUniformLocation(*program, src.as_ptr()) as UniformLocation
        }
    }

    fn uniform_matrix_4fv(&self, location: &UniformLocation, size: i32, transpose: bool, matrix: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(*location as i32, size, transpose as u8, matrix as *const na::Matrix<f32, na::U4, na::U4, na::MatrixArray<f32, na::U4, na::U4>> as *const f32);
        }
    }

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(type_, first, count);
        }
    }

    fn draw_elements(&self, mode: GLEnum, count: i32, type_: GLEnum, offset: GLintptr) {
        unsafe { 
            gl::DrawElements(mode, count, type_, mem::transmute(offset)) 
        }
    }
}