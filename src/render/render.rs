extern crate nalgebra_glm as glm;

use crate::engine::Engine;
//use crate::consts::*;
use crate::tsu;
//use gl::types::*;
use gl::*;

use super::model::{Mesh, Model};
use super::shaders::ShaderProgram;
use super::texture::Texture;

const VERTS: [f32; 12] = [
     0.5,  0.5, 0.0,
     0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0,
    -0.5,  0.5, 0.0,
];
const INDICES: [u32; 6] = [
    0, 1, 2, 2, 0, 3
];
const UVS: [f32; 8] = [
    1.0, 0.0,
    1.0, 1.0,
    0.0, 1.0,
    0.0, 0.0
];

pub struct Renderer {
    frames: u128,
    models: Vec<Model>
}

impl Renderer {
    pub fn init() -> Renderer {
        let shader = ShaderProgram::quick_load("/default");
        let mut model = Model::create(shader);

        let texture = Texture::load("/ayame", REPEAT, LINEAR_MIPMAP_LINEAR);
        model.set_texture(texture);

        let mesh = Mesh {
            verteces: VERTS.to_vec(),
            indeces: INDICES.to_vec(),
            uvs: UVS.to_vec()
        };
        model.set_mesh(mesh);

        let mut models: Vec<Model> = Vec::new();
        models.push(model);

        Renderer {
            frames: 0,
            models
        }
    }

    pub fn render(&mut self) {
        unsafe {
            let (r, g, b, a) = tsu::hex_to_floats(0x00000000);
            ClearColor(r, g, b, a);
            Clear(COLOR_BUFFER_BIT);

            for m in &mut self.models {
                let p = glm::vec3(0.0, 0.2, 0.0) * Engine::get_time().sin();
                m.set_position(p);
                m.draw();
            }

            self.frames += 1;
        }
    }
}
