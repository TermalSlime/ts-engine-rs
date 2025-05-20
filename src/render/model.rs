extern crate nalgebra_glm as glm;
use gl::*;
use nalgebra_glm::quat_angle;
use nalgebra_glm::quat_axis;
use std::ffi::c_void;
use std::mem::transmute;

use crate::engine::Engine;

use super::texture::Texture;
use super::buffers::*;
use super::shaders::*;

struct RenderData {
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
}

pub struct Mesh {
    pub verteces: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indeces: Vec<u32>,
}

pub struct Model {
    mesh: Mesh,

    texture: Option<Texture>,
    render_data: RenderData,
    shader: ShaderProgram,

    transformation_matrix: glm::Mat4,

    position: glm::Vec3,
    quaternion: glm::Quat,
    scale: glm::Vec3,
}

impl Mesh {
    pub fn create() -> Mesh {
        Mesh {
            verteces: Vec::new(),
            indeces: Vec::new(),
            uvs: Vec::new()
        }
    }
    fn get_vertex_data(&self) -> Option<Vec<f32>> {
        if self.verteces.len() as f32 / 3.0 != self.uvs.len() as f32 / 2.0 {
            return None;
        }

        let mut verteces = self.verteces.clone();
        let mut uvs = self.uvs.clone();
        let mut data: Vec<f32> = Vec::new();

        let len = self.verteces.len() / 3usize;

        for i in 0..len {
            let vertx_drain = verteces.drain(0..3);
            let uv_drain = uvs.drain(0..2);

            for v in vertx_drain {
                data.push(v);
            }
            for u in uv_drain {
                data.push(u);
            }
        }

        Some(data)
    }
    fn get_indeces(&self) -> Vec<u32> {
        self.indeces.clone()
    }
}

impl Model {
    pub fn create(mut shader: ShaderProgram) -> Model {
        let position = glm::vec3(0.0, 0.0, 0.0);
        let quaternion = glm::Quat::identity();
        let scale = glm::vec3(1.0, 1.0, 1.0);

        let matrix = glm::Mat4::identity();

        let vao = VAO::init();
        vao.bind();

        let vbo = VBO::init();
        let ebo = EBO::init();
        vbo.bind();
        ebo.bind();

        let pos_attr = ShaderAttribute {
            name: "aPos".to_string(),
            type_: FLOAT,
            size: 3,
            normalized: false,
        };
        let uv_attr = ShaderAttribute {
            name: "aUV".to_string(),
            type_: FLOAT,
            size: 2,
            normalized: false,
        };

        shader.add_shader_attribute(pos_attr);
        shader.add_shader_attribute(uv_attr);

        shader.apply_shader_attributes();
        shader.bind_frag_data_location("FragColor".to_string());

        let render_data = RenderData {
            vao,
            vbo,
            ebo
        };

        return Model {
            mesh: Mesh::create(),
            texture: None,
            render_data,
            shader,
            transformation_matrix: matrix,
            position,
            quaternion,
            scale
        };
    }

    pub fn draw(&self)
    {
        self.render_data.vao.bind();
        self.shader.use_program();

        match &self.texture {
            Some(texture) => {
                texture.bind();
            }
            _ => {}
        }

        unsafe {
            let time_loc = self.shader.get_uniform_location("time".to_string());
            Uniform1f(time_loc, Engine::get_time());

            let transform_loc = self.shader.get_uniform_location("transform".to_string());

            let mat_ptr = transmute(&self.transformation_matrix[0]);
            UniformMatrix4fv(transform_loc, 1, FALSE, mat_ptr);

            DrawElements(
                TRIANGLES,
                self.mesh.indeces.len() as i32,
                UNSIGNED_INT,
                0 as *const c_void);
        }
    }

    pub fn set_position(&mut self, pos: glm::Vec3) {
        self.position = pos;
        self.update_matrix();
    }
    pub fn get_position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn set_scale(&mut self, scl: glm::Vec3) {
        self.scale = scl;
        self.update_matrix();
    }
    pub fn get_scale(&self) -> glm::Vec3 {
        self.scale
    }

    pub fn set_quaternion(&mut self, qtr: glm::Quat) {
        self.quaternion = qtr;
        self.update_matrix();
    }
    pub fn get_quaternion(&self) -> glm::Quat{
        self.quaternion
    }

    fn update_matrix(&mut self) {
        let mut matrix = glm::Mat4::identity();

        matrix = glm::translate(&matrix, &self.position);
        matrix = glm::scale(&matrix, &self.scale);

        let qtr = self.quaternion;

        let ang = quat_angle(&qtr);
        let axis = quat_axis(&qtr);

        matrix = glm::rotate(&matrix, ang, &axis);

        self.transformation_matrix = matrix;
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = Some(texture);
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;

        let vert_data = self.mesh.get_vertex_data().expect("failed to generate vertex data");
        self.render_data.vbo.put_data(
            &vert_data,
            STATIC_DRAW
        );

        let index_data = self.mesh.get_indeces();
        self.render_data.ebo.put_data(
            &index_data,
            STATIC_DRAW
        );
    }
}
