mod app;
mod engine;
mod flowfield;
mod instance_group;
mod object;
mod shape;
mod vertex;

extern crate glium;
extern crate noise;
extern crate rand;
extern crate rayon;
extern crate winit;

use app::App;
use engine::{Engine, Updatable};

fn main() {
    let mut app = App::default_app();
    let engine: Engine = Engine::init(&app.event_loop_ref());
    app.run(engine);
}
