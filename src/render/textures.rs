use gl::*;
use gl::types::*;
use stb_image_rust::*;
use crate::file_loader::*;
use crate::consts;
use super::render::*;

pub struct Texture {
    ptr: u32
}

impl Texture {
    pub fn load(path: &str, repeat_mode: GLenum, filter_mode: GLenum) -> Texture {
        unsafe {
            let mut ptr: u32 = 0;
            GenTextures(1, &mut ptr);

            BindTexture(TEXTURE_2D, ptr);

            TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, repeat_mode as i32);
            TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, repeat_mode as i32);
            TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, filter_mode as i32);

            match filter_mode {
                NEAREST | NEAREST_MIPMAP_LINEAR | NEAREST_MIPMAP_NEAREST => {
                    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
                }
                LINEAR | LINEAR_MIPMAP_LINEAR | LINEAR_MIPMAP_NEAREST => {
                    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
                }
                _ => { panic!() }
            }

            let mut data = read_texture(path).expect("failed to load texture");

            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut comp: i32 = 0;

            let img = stbi_load_from_memory(
                data.as_mut_ptr(),
                data.len() as i32,
                &mut width,
                &mut height,
                &mut comp,
                0
            );

            TexImage2D(
                TEXTURE_2D,
                0,
                RGBA as i32,
                width,
                height,
                0,
                RGBA,
                UNSIGNED_BYTE,
                img as *const _);

            GenerateMipmap(TEXTURE_2D);

            //stbi_image_free(img);

            Texture { ptr }
        }
    }
    pub fn bind(&self) {
        unsafe {
            BindTexture(TEXTURE_2D, self.ptr);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            DeleteTextures(1, [self.ptr].as_ptr());
        }
    }
}
