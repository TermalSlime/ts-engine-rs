mod consts;
mod engine;
mod render;
mod tsu;
mod file_loader;

use engine::Engine;
use render::render::*;

fn main() {
    let engine: Engine = Engine::init();
    engine.run_loop();
}
