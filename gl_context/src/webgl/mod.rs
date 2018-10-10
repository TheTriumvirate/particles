/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */
#![allow(unused_results)]

// Ignore warnings in autogenerated bindings
#[allow(unused_parens)]
#[allow(dead_code)]
#[cfg(target_arch = "wasm32")]
mod webgl_bindings;

use shaders::ShaderType;

use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, IParentNode, TypedArray};
use stdweb::Value;

use std::{mem};

use na::{Matrix4};

use context::{GlPrimitive, GlPrimitiveArray};

use Program;
use Shader;
use AbstractContext;
use NativeBuffer;
use Context;
use NativeTexture;

use self::webgl_bindings::{
    WebGLRenderingContext, WebGLBuffer, WebGLProgram,
    WebGLShader, WebGLUniformLocation, WebGLTexture
};

pub use self::webgl_bindings::{
    GLenum, GLintptr, GLsizeiptr
};

pub type GLShader = WebGLShader;
pub type GLProgram = WebGLProgram;
pub type UniformLocation = WebGLUniformLocation;
pub type GLEnum = GLenum;
pub type GLBuffer = WebGLBuffer;
pub type GLUint = u32;
pub type GLTexture = WebGLTexture;

lazy_static! {
    static ref CONTEXT: Context = WebGLContext::new();
}

pub struct WebGLContext {
    context: WebGLRenderingContext
}

impl WebGLContext {
    fn new() -> Self {
        let canvas: CanvasElement = document()
            .query_selector("#canvas")
            .expect("No canvas found")
            .unwrap()
            .try_into()
            .unwrap();

        let context = js!(return @{canvas}.getContext("webgl", {alpha: false});).try_into().unwrap();
        WebGLContext { context }
    }
}

impl AbstractContext for WebGLContext {
    const FLOAT: u32 = WebGLRenderingContext::FLOAT;
    const COLOR_BUFFER_BIT: u32 = WebGLRenderingContext::COLOR_BUFFER_BIT;
    const VERTEX_SHADER: u32 = WebGLRenderingContext::VERTEX_SHADER;
    const FRAGMENT_SHADER: u32 = WebGLRenderingContext::FRAGMENT_SHADER;
    const ARRAY_BUFFER: u32 = WebGLRenderingContext::ARRAY_BUFFER;
    const ELEMENT_ARRAY_BUFFER: u32 = WebGLRenderingContext::ELEMENT_ARRAY_BUFFER;
    const STATIC_DRAW: u32 = WebGLRenderingContext::STATIC_DRAW;
    const DYNAMIC_DRAW: u32 = WebGLRenderingContext::STATIC_DRAW;
    const COMPILE_STATUS: u32 = WebGLRenderingContext::COMPILE_STATUS;
    const POINTS: u32 = WebGLRenderingContext::POINTS;
    const LINE_STRIP: u32 = WebGLRenderingContext::LINE_STRIP;
    const LINE_LOOP: u32 = WebGLRenderingContext::LINE_LOOP;
    const LINES: u32 = WebGLRenderingContext::LINES;
    const TRIANGLE_STRIP: u32 = WebGLRenderingContext::TRIANGLE_STRIP;
    const TRIANGLE_FAN: u32 = WebGLRenderingContext::TRIANGLE_FAN;
    const TRIANGLES: u32 = WebGLRenderingContext::TRIANGLES;
    const UNSIGNED_SHORT: u32 = WebGLRenderingContext::UNSIGNED_SHORT;
    const TEXTURE_2D: u32 = WebGLRenderingContext::TEXTURE_2D;
    const UNSIGNED_BYTE: u32 = WebGLRenderingContext::UNSIGNED_BYTE;
    const RGBA: u32 = WebGLRenderingContext::RGBA;
    const LUMINANCE: u32 = WebGLRenderingContext::LUMINANCE;
    const TEXTURE0: u32 = WebGLRenderingContext::TEXTURE0;
    const TEXTURE_WRAP_S: u32 = WebGLRenderingContext::TEXTURE_WRAP_S;
    const TEXTURE_WRAP_T: u32 = WebGLRenderingContext::TEXTURE_WRAP_T;
    const CLAMP_TO_EDGE: u32 = WebGLRenderingContext::CLAMP_TO_EDGE;
    const TEXTURE_MIN_FILTER: u32 = WebGLRenderingContext::TEXTURE_MIN_FILTER;
    const TEXTURE_MAG_FILTER: u32 = WebGLRenderingContext::TEXTURE_MAG_FILTER;
    const LINEAR: u32 = WebGLRenderingContext::LINEAR;
    const UNPACK_ALIGNMENT: u32 = WebGLRenderingContext::UNPACK_ALIGNMENT;
    const DEPTH_BUFFER_BIT: u32 = WebGLRenderingContext::DEPTH_BUFFER_BIT;
    const FRONT_AND_BACK: u32 = WebGLRenderingContext::FRONT_AND_BACK;
    const DEPTH_TEST: u32 = WebGLRenderingContext::DEPTH_TEST;
    const UNSIGNED_INT: u32 = WebGLRenderingContext::UNSIGNED_INT;

    fn get_context() -> &'static Context {
        &CONTEXT
    }
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
    }

    fn clear(&self, mask: u32) {
        self.context.clear(mask);
    }
    fn create_shader(&self, type_: ShaderType) -> Option<Shader> {
        match type_ {
            ShaderType::Vertex => self.context.create_shader(Self::VERTEX_SHADER),
            ShaderType::Fragment => self.context.create_shader(Self::FRAGMENT_SHADER),
        }
    }

    fn shader_source(&self, shader: &Shader, source: &str) {
        self.context.shader_source(shader, source);
    }

    fn compile_shader(&self, shader: &Shader) {
        self.context.compile_shader(shader);
    }

    fn delete_shader(&self, shader: &Shader) {
        self.context.delete_shader(Some(shader));
    }

    fn get_shader_parameter(&self, shader: &Shader, pname: GLEnum) -> Option<i32> {
        // TODO: Handle all value types?
        match self.context.get_shader_parameter(shader, pname) {
            Value::Number(n) => n.try_into().ok(),
            _ => None,
        }
    }

    fn get_shader_info_log(&self, shader: &Shader) -> Option<String> {
        self.context.get_shader_info_log(shader)
    }

    fn create_program(&self) -> Option<Program> {
        self.context.create_program()
    }

    fn attach_shader(&self, program: &Program, shader: &Shader) {
        self.context.attach_shader(program, shader);
    }

    fn link_program(&self, program: &Program) {
        self.context.link_program(program);
    }

    fn use_program(&self, program: &Program) {
        self.context.use_program(Some(program));
    }

    fn delete_program(&self, program: &Program) {
        self.context.delete_program(Some(program));
    }

    fn create_buffer(&self) -> Option<NativeBuffer> {
        self.context.create_buffer()
    }

    fn bind_buffer(&self, target: GLEnum, buffer: &NativeBuffer) {
        self.context.bind_buffer(target, Some(buffer));
    }

    fn buffer_data<T: GlPrimitive>(&self, target: GLEnum, data: &[T], usage: GLEnum) {
        match T::into(data) {
            GlPrimitiveArray::F32(data) => {
                let abuf = TypedArray::<f32>::from(data);
                self.context
                    .buffer_data_1(target, Some(&abuf.buffer()), usage);
            },
            GlPrimitiveArray::U16(data) => {
                let abuf = TypedArray::<u16>::from(data);
                self.context
                    .buffer_data_1(target, Some(&abuf.buffer()), usage);
            },
            GlPrimitiveArray::U32(data) => {
                let abuf = TypedArray::<u32>::from(data);
                self.context
                    .buffer_data_1(target, Some(&abuf.buffer()), usage);
            }
        }
    }

    fn delete_buffer(&self, buffer: &NativeBuffer) {
        self.context.delete_buffer(Some(buffer));
    }

    fn get_attrib_location(&self, program: &Program, name: &str) -> GLUint {
        self.context.get_attrib_location(program, name) as u32
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
        self.context.vertex_attrib_pointer(
            *pointer,
            size,
            type_,
            normalized,
            (stride * mem::size_of::<f32>() as i32) as i32,
            (offset * mem::size_of::<f32>() as i32) as GLintptr,
        ) // todo: offset as custom type
    }

    fn enable_vertex_attrib_array(&self, pointer: &GLUint) {
        self.context.enable_vertex_attrib_array(*pointer)
    }

    fn disable_vertex_attrib_array(&self, pointer: &GLUint) {
        self.context.disable_vertex_attrib_array(*pointer)
    }
    
    fn bind_attrib_location(&self, program: &Program, index: GLUint, name: &str) {
        self.context.bind_attrib_location(program, index, name)
    }

    fn get_uniform_location(&self, program: &Program, name: &str) -> UniformLocation {
        self.context.get_uniform_location(program, name).expect("Uniform location could not be found or does not exist")
    }

    fn uniform_matrix_4fv(&self, location: &UniformLocation, _size: i32, transpose: bool, matrix: &Matrix4<f32>) {
        self.context.uniform_matrix4fv(Some(location), transpose, matrix.as_slice())
    }
    
    fn uniform1i(&self, location: &UniformLocation, x: i32) {
        self.context.uniform1i(Some(location), x);
    }
    
    fn uniform1f(&self, location: &UniformLocation, x: f32) {
        self.context.uniform1f(Some(location), x);
    }
    
    fn uniform3f(&self, location: &UniformLocation, x: f32, y: f32, z: f32) {
        self.context.uniform3f(Some(location), x, y, z);
    }
    
    fn create_texture(&self) -> Option<NativeTexture> {
        self.context.create_texture()
    }

    fn bind_texture(&self, target: GLEnum, texture: &NativeTexture) {
        self.context.bind_texture(target, Some(texture));
    }
    
    fn unbind_texture(&self, target: GLEnum) {
        self.context.bind_texture(target, None);
    }
    
    fn tex_parameteri(&self, target: GLEnum, pname: GLEnum, param: i32) {
        self.context.tex_parameteri(target, pname, param)
    }

    fn tex_image2d(
        &self,
        target: GLenum,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: GLenum,
        pixels: Option<&[u8]>,
    ) {
        match pixels {
            Some(pixels) => self.context.tex_image2_d(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                Self::UNSIGNED_BYTE,
                Some(pixels),
            ),
            None => self.context.tex_image2_d(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                Self::UNSIGNED_BYTE,
                None::<&TypedArray<u8>>,
            ),
        }
    }

    fn tex_sub_image2d(
        &self,
        target: GLenum,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: GLenum,
        pixels: Option<&[u8]>,
    ) {
        match pixels {
            Some(pixels) => self.context.tex_sub_image2_d(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                Self::UNSIGNED_BYTE,
                Some(pixels),
            ),
            None => self.context.tex_sub_image2_d(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                Self::UNSIGNED_BYTE,
                None::<&TypedArray<u8>>,
            ),
        }
    }

    fn delete_texture(&self, texture: &NativeTexture) {
        self.context.delete_texture(Some(texture));
    }
    
    fn active_texture(&self, _type: GLEnum) {
        self.context.active_texture(_type);
    }
    
    fn generate_mipmap(&self, target: GLEnum) {
        self.context.generate_mipmap(target);
    }

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32) {
        self.context.enable(WebGLRenderingContext::BLEND);
        self.context.blend_func(
            WebGLRenderingContext::SRC_ALPHA,
            WebGLRenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        self.context.draw_arrays(type_, first, count)
    }

    fn draw_elements(&self, mode: GLEnum, count: i32, type_: GLEnum, offset: GLintptr) {
        self.context.draw_elements(mode, count, type_, offset);
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.context.viewport(x, y, width, height);
    }
    

    fn pixel_storei(&self, pname: GLenum, param: i32) {
        self.context.pixel_storei(pname, param)
    }

    fn enable(&self, cap: GLEnum) {
        self.context.enable(cap);
    }

    fn disable(&self, cap: GLEnum) {
        self.context.disable(cap);
    }
}