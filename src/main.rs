mod landscape;
mod util;
mod runnable;
mod drawable;

extern crate glium;
extern crate noise;
extern crate rand;
extern crate rayon;
extern crate winit;

use runnable::app::App;
use runnable::engine::{Engine, Updatable};

fn main() {
    let mut app = App::default_app();
    let engine: Engine = Engine::init(&app.event_loop_ref());
    app.run(engine);
}
