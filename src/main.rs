extern crate core;
extern crate glium;
extern crate noise;
extern crate rand;
extern crate rayon;
extern crate winit;

mod boids;
mod drawable;
mod landscape;
mod runnable;
mod util;
mod gol;

use runnable::app::App;
use runnable::engine::{Engine, Updatable};

fn main() {
    let mut app = App::default_app();
    let engine: Engine = Engine::init(&app.event_loop_ref());
    app.run(engine);
}
