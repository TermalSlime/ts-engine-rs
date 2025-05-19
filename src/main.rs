mod engine;
mod render;
mod tsu;
mod consts;
use engine::Engine;

fn main() {
    let engine: Engine = Engine::init();

    engine.run_loop();
}
