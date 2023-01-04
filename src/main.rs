mod app;
pub mod shape;
mod vertex;
mod engine;
extern crate glium;

use app::{App};
use engine::{Engine, Runnable};
use shape::Shape;

fn main() {
    let app = App::default_app();
    let engine: Engine<Shape> = Engine::init(&app.event_loop);
    app.run(engine);
}
