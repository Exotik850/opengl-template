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
use vertex::{Manipulate, Vertex};

fn main() {
    let app = App::default_app();
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
    };
    let vertices = vec![vertex1, vertex2, vertex3, vertex2 * 2.0f32];
    let index_type = PrimitiveType::TrianglesList;

    let shape = Shape::new(&app.display, vertices, index_type, 0);
    let mut shape2 = shape.clone(&app.display);
    shape2.rotate(45.0);

    let shapes: Vec<Shape> = vec![shape, shape2];

    let program = Program::from_source(&app.display, BASE_VSHADER, BASE_FSHADER, None).unwrap();
    let programs = vec![program];

    let engine: Engine<Shape> = Engine::new(shapes, programs);
    app.run(engine);
}
