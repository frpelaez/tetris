use engine::Engine;
use interface::Interface;

#[allow(dead_code)]
mod engine;
mod interface;

fn main() {
    println!("Tetris!");

    let engine = Engine::new();

    Interface::run(engine)
}
