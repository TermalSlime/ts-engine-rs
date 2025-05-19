use crate::render::render::Renderer;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::video::{GLContext, GLProfile, Window};
use sdl3::Sdl;
use sdl3::VideoSubsystem;

extern crate sdl3;

pub struct Engine {
    sdl_context: Sdl,
    video_subsistem: VideoSubsystem,
    window: Window,
    render_context: GLContext,
    renderer: Renderer,
}

impl Engine {
    pub fn init() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsistem = sdl_context.video().unwrap();

        let window = Self::create_window(&video_subsistem);
        let render_context = Self::init_opengl(&video_subsistem, &window);

        Engine {
            sdl_context,
            video_subsistem,
            window,
            render_context,
            renderer: Renderer::init(),
        }
    }

    pub fn run_loop(self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            self.renderer.render_frame();

            self.window.gl_swap_window();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
        }
    }

    fn create_window(video_subsistem: &VideoSubsystem) -> Window {
        let window = video_subsistem
            .window("title", 800, 600)
            .opengl()
            .position_centered()
            .build()
            .unwrap();
        window
    }

    fn init_opengl(video_subsistem: &VideoSubsystem, window: &Window) -> GLContext {
        let gl_attr = video_subsistem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        gl::load_with(|p| video_subsistem.gl_get_proc_address(p).unwrap() as _);

        let render_context = window.gl_create_context().unwrap();
        render_context
    }
}
