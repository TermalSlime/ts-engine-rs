use std::mem::{size_of, transmute};

use gl::*;
use gl::types::*;

pub struct VBO {
    ptr: u32,
}

pub struct VAO {
    ptr: u32,
}

pub struct EBO {
    ptr: u32
}

impl VBO {
    pub fn init() -> VBO {
        unsafe {
            let mut ptr = 0;
            GenBuffers(1, &mut ptr);
            VBO { ptr }
        }
    }
    pub fn bind(&self) {
        unsafe {
            BindBuffer(ARRAY_BUFFER, self.ptr);
        }
    }
    pub fn put_data(&self, data: &Vec<f32>, usage: GLenum) {
        self.bind();
        unsafe {
            BufferData(
                ARRAY_BUFFER,
                (data.len() * size_of::<GLfloat>()) as GLsizeiptr,
                transmute(&data[0]),
                usage,
            );
        }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            DeleteBuffers(1, self.ptr as *const u32);
        }
    }
}

impl VAO {
    pub fn init() -> VAO {
        unsafe {
            let mut ptr = 0;
            GenVertexArrays(1, &mut ptr);
            VAO { ptr }
        }
    }
    pub fn bind(&self) {
        unsafe {
            BindVertexArray(self.ptr);
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            DeleteVertexArrays(1, self.ptr as *const u32);
        }
    }
}

impl EBO {
    pub fn init() -> EBO {
        unsafe {
            let mut ptr = 0;
            GenBuffers(1, &mut ptr);
            EBO { ptr }
        }
    }
    pub fn bind(&self) {
        unsafe {
            BindBuffer(ELEMENT_ARRAY_BUFFER, self.ptr);
        }
    }
    pub fn put_data(&self, data: &Vec<u32>, usage: GLenum) {
        self.bind();
        unsafe {
            BufferData(
                ELEMENT_ARRAY_BUFFER,
                (data.len() * size_of::<GLuint>()) as GLsizeiptr,
                transmute(&data[0]),
                usage,
            );
        }
    }
}

impl Drop for EBO {
    fn drop(&mut self) {
        unsafe {
            DeleteBuffers(1, self.ptr as *const u32);
        }
    }
}
