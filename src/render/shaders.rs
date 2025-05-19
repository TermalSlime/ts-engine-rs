use gl::types::*;
use gl::*;
use std::str;

use std::{
    collections::HashMap, ffi::{c_void, CString}, mem, ptr
};

pub const EXM_VSHADER: &str =
"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aCol;

out vec4 vCol;

void main()
{
gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
vCol = aCol;
}";

pub const EXM_FSHADER: &str =
"#version 330 core
#define PI 3.1415926538
out vec4 FragColor;

in vec4 vCol;

uniform float time;

void main()
{
float red = (sin(time) / 2.0f) + 0.5f;
float gre = (sin(time + PI * 2 * 0.33) / 2.0f) + 0.5f;
float blu = (sin(time + PI * 2 * 0.66) / 2.0f) + 0.5f;
FragColor = vec4(red, gre, blu, vCol.a);
}";

pub struct VertexShader {
    ptr: u32,
}

impl VertexShader {
    pub fn compile(src: &str) -> Option<VertexShader> {
        let shader: u32;

        unsafe {
            shader = CreateShader(VERTEX_SHADER);

            let c_str = CString::new(src.as_bytes()).unwrap();
            ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            CompileShader(shader);

            let mut status = FALSE as GLint;
            GetShaderiv(shader, COMPILE_STATUS, &mut status);

            if status != (TRUE as GLint) {
                let mut len = 0;
                GetShaderiv(shader, INFO_LOG_LENGTH, &mut len);
                println!("{}", len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);

                GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                let log = String::from_utf8(buf).expect("unable to decode shader log");
                println!("{:?}", log);

                return None;
            }
        }

        return Some(VertexShader { ptr: shader });
    }
}

pub struct FragmentShader {
    ptr: u32,
}

impl FragmentShader {
    pub fn compile(src: &str) -> Option<FragmentShader> {
        let shader: u32;

        unsafe {
            shader = CreateShader(FRAGMENT_SHADER);

            let c_str = CString::new(src.as_bytes()).unwrap();
            ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            CompileShader(shader);

            let mut status = FALSE as GLint;
            GetShaderiv(shader, COMPILE_STATUS, &mut status);

            if status != (TRUE as GLint) {
                let mut len = 0;
                GetShaderiv(shader, INFO_LOG_LENGTH, &mut len);
                println!("{}", len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);

                GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                let log = String::from_utf8(buf).expect("unable to decode shader log");
                println!("{:?}", log);

                return None;
            }
        }

        return Some(FragmentShader { ptr: shader });
    }
}

pub struct ShaderAttribute {
    pub name: String,
    pub type_: GLenum,
    pub size: u32,
    pub normalized: bool,
}

#[derive(Default)]
pub struct ShaderProgram {
    pub ptr: u32,
    pub attributes: Vec<ShaderAttribute>,
}

impl ShaderProgram {
    pub fn link_program(vs: &VertexShader, fs: &FragmentShader) -> Option<ShaderProgram> {
        unsafe {
            let program = CreateProgram();
            AttachShader(program, vs.ptr);
            AttachShader(program, fs.ptr);
            LinkProgram(program);

            let mut status = FALSE as GLint;
            GetProgramiv(program, LINK_STATUS, &mut status);

            if status != (TRUE as GLint) {
                let mut len = 0;
                GetProgramiv(program, INFO_LOG_LENGTH, &mut len);
                println!("{}", len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                return None;
            }
            return Some(ShaderProgram {
                ptr: program,
                ..Default::default()
            });
        }
    }

    pub fn use_program(&self) {
        unsafe {
            UseProgram(self.ptr);
        }
    }

    pub fn bind_frag_data_location(&self, name: String) {
        unsafe {
            BindFragDataLocation(self.ptr, 0, CString::new(name).unwrap().as_ptr());
        }
    }

    pub fn add_shader_attribute(&mut self, shader_attribute: ShaderAttribute) {
        self.attributes.push(shader_attribute);
    }

    pub fn apply_shader_attributes(&self) {
        let mut size = 0;

        for attr in &self.attributes {
            size += (attr.size as usize * get_mem_size_of_gl_type(attr.type_)) as i32;
        }

        let mut offset = 0;

        for attr in &self.attributes {
            unsafe {
                let attr_ptr =
                    GetAttribLocation(self.ptr, CString::new(attr.name.clone()).unwrap().as_ptr());
                EnableVertexAttribArray(attr_ptr as GLuint);
                VertexAttribPointer(
                    attr_ptr as GLuint,
                    attr.size as i32,
                    FLOAT,
                    attr.normalized as GLboolean,
                    size,
                    offset as *const c_void,
                );
                // stride is size of one memory block (xyzrgb|xyzrgb|...) = 6
                // pointer is offset in one memory block (|xyz|rgbxyzrgb) = 0, (xyz|rgb|xyzrgb) = 3
                offset += (attr.size as usize * get_mem_size_of_gl_type(attr.type_)) as i32
            }
        }
    }

    pub fn get_uniform_location(&self, name: String) -> i32{
        unsafe {
            GetUniformLocation(self.ptr, CString::new(name).unwrap().as_ptr())
        }
    }
}

fn get_mem_size_of_gl_type(gl_type: GLenum) -> usize {
    match gl_type {
        BOOL => mem::size_of::<GLboolean>(),
        BYTE => mem::size_of::<GLbyte>(),
        UNSIGNED_BYTE => mem::size_of::<GLubyte>(),
        SHORT => mem::size_of::<GLshort>(),
        UNSIGNED_SHORT => mem::size_of::<GLushort>(),
        INT => mem::size_of::<GLint>(),
        UNSIGNED_INT => mem::size_of::<GLuint>(),
        FLOAT => mem::size_of::<GLfloat>(),
        DOUBLE => mem::size_of::<GLdouble>(),
        _ => 0,
    }
}
