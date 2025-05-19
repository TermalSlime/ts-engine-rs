use gl::*;
use gl::types::*;

use crate::consts;

struct Texture {
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
            TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, filter_mode as i32);

            Texture { ptr }
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            DeleteTextures(1, self.ptr as *const u32);
        }
    }
}
