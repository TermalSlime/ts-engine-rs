mod consts;
mod engine;
mod render;
mod tsu;
mod file_loader;

use engine::Engine;

fn main() {
    let engine: Engine = Engine::init();

    engine.run_loop();
}
