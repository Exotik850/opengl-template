mod app;
mod engine;
mod shape;
mod vertex;

extern crate glium;
extern crate winit;

use app::App;
use engine::{Engine, Runnable, BASE_FSHADER, BASE_VSHADER};
use glium::index::PrimitiveType;
use glium::Program;
use shape::{Drawable, Shape};

fn main() {
    let mut app = App::default_app();
    let engine: Engine<Shape> = Engine::init(app.event_loop_ref());
    app.run(engine);
}
