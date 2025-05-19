use crate::consts::{self, COOL_COLOR};
use crate::tsu;
use gl::types::*;
use gl::*;
use std::ffi::c_void;
use std::mem::{size_of, transmute};

use super::shaders::{self, FragmentShader, ShaderAttribute, ShaderProgram, VertexShader};

const VERTS: [f32; 21] = [
    -0.5, -0.5, 0.0,  0.6, 0.3, 0.7, 1.0,
     0.5, -0.5, 0.0,  0.6, 0.3, 0.7, 1.0,
     0.0,  0.5, 0.0,  0.4, 0.3, 0.8, 1.0,
];
const INDICES: [u32; 3] = [
    0, 1, 2
];

struct VBO {
    ptr: u32,
}

struct VAO {
    ptr: u32,
}

struct EBO {
    ptr: u32
}

impl VBO {
    fn init() -> VBO {
        unsafe {
            let mut ptr = 0;
            GenBuffers(1, &mut ptr);
            VBO { ptr }
        }
    }
    fn bind(&self) {
        unsafe {
            BindBuffer(ARRAY_BUFFER, self.ptr);
        }
    }
    fn put_data(&self, data: &Vec<f32>, usage: GLenum) {
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
    fn init() -> VAO {
        unsafe {
            let mut ptr = 0;
            GenVertexArrays(1, &mut ptr);
            VAO { ptr }
        }
    }
    fn bind(&self) {
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
    fn init() -> EBO {
        unsafe {
            let mut ptr = 0;
            GenBuffers(1, &mut ptr);
            EBO { ptr }
        }
    }
    fn bind(&self) {
        unsafe {
            BindBuffer(ELEMENT_ARRAY_BUFFER, self.ptr);
        }
    }
    fn put_data(&self, data: &Vec<u32>, usage: GLenum) {
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

pub struct Renderer {
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
    program: ShaderProgram,
}

impl Renderer {
    pub fn init() -> Renderer {
        let vao = VAO::init();
        vao.bind();

        let vbo = VBO::init();
        vbo.put_data(&VERTS.to_vec(), STATIC_DRAW);

        let ebo = EBO::init();
        ebo.put_data(&INDICES.to_vec(), STATIC_DRAW);

        let vshader =
            VertexShader::compile(shaders::EXM_VSHADER).expect("could not compile vertex shader");
        let fshader = FragmentShader::compile(shaders::EXM_FSHADER)
            .expect("could not compile fragment shader");

        let mut program =
            ShaderProgram::link_program(&vshader, &fshader).expect("could not ling program");

        let pos_attr = ShaderAttribute {
            name: "aPos".to_string(),
            type_: FLOAT,
            size: 3,
            normalized: false,
        };
        let col_attr = ShaderAttribute {
            name: "aCol".to_string(),
            type_: FLOAT,
            size: 4,
            normalized: false,
        };

        program.add_shader_attribute(pos_attr);
        program.add_shader_attribute(col_attr);
        program.apply_shader_attributes();
        program.bind_frag_data_location("FragColor".to_string());

        Renderer {
            vao,
            vbo,
            ebo,
            program
        }
    }

    pub fn render_frame(&self) {
        unsafe {
            let (r, g, b, a) = tsu::hex_to_floats(0xffffffff);
            ClearColor(r, g, b, a);
            Clear(COLOR_BUFFER_BIT);

            self.program.use_program();
            self.program.apply_shader_attributes();
            self.vao.bind();
            DrawElements(TRIANGLES, INDICES.len() as i32, UNSIGNED_INT, 0 as *const c_void);
        }
    }
}
