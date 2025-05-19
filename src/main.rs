mod consts;
mod engine;
mod render;
mod tsu;

use engine::Engine;
use tsu::hex_to_floats;

fn main() {
    let mut engine: Engine = Engine::init();

    engine.run_loop();
}
