//use crate::consts::*;
use crate::render::buffers::*;
use crate::tsu;
//use gl::types::*;
use gl::*;
use std::ffi::c_void;
use sdl3::sys::timer::SDL_GetTicks;

use super::shaders::{ShaderAttribute, ShaderProgram};

const VERTS: [f32; 21] = [
    -0.5, -0.5, 0.0,  0.6, 0.3, 0.7, 1.0,
     0.5, -0.5, 0.0,  0.6, 0.3, 0.7, 1.0,
     0.0,  0.5, 0.0,  0.4, 0.3, 0.8, 1.0,
];
const INDICES: [u32; 3] = [
    0, 1, 2
];
const TEX_COORDS: [f32; 6] = [
    0.0, 0.0,
    1.0, 0.0,
    0.5, 1.0
];

pub struct Renderer {
    frames: u128,
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

        let mut program = ShaderProgram::quick_load("/default");

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
        program.bind_frag_data_location("FragColor".to_string());

        Renderer {
            frames: 0,
            vao,
            vbo,
            ebo,
            program
        }
    }

    pub fn render(&mut self) {
        unsafe {
            let time = SDL_GetTicks() as f32 / 1000 as f32;
            //println!("{time}");

            let (r, g, b, a) = tsu::hex_to_floats(0xffffffff);
            ClearColor(r, g, b, a);
            Clear(COLOR_BUFFER_BIT);

            let time_loc = self.program.get_uniform_location("time".to_string());

            self.program.use_program();
            self.program.apply_shader_attributes();
            self.vao.bind();

            Uniform1f(time_loc, time);

            DrawElements(TRIANGLES, INDICES.len() as i32, UNSIGNED_INT, 0 as *const c_void);

            self.frames += 1;
        }
    }
}
