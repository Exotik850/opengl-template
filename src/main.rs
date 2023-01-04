mod app;
pub mod shape;
mod vertex;
mod engine;
extern crate glium;

use glium::glutin::event_loop::EventLoop;
use app::{App};
use engine::{Engine, Runnable};

fn main() {
    let app = App::default_app();
    let engine = Engine::default_engine(&app.event_loop);
    app.run(engine);
}
