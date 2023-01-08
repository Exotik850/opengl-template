mod app;
mod engine;
mod shape;
mod vertex;
mod object;

extern crate glium;
extern crate winit;

use app::App;
use engine::{Engine, Updatable};

fn main() {
    let mut app = App::default_app();
    let engine: Engine = Engine::init(&app.event_loop_ref());
    app.run(engine);
}