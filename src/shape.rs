use crate::vertex::{Manipulate, Vertex, VertexArray};
use glium::index::PrimitiveType;
use glium::{uniform, Display, Frame, Program, Surface};

pub struct Shape {
    pub id: u32,
    pub va: VertexArray,
    pub indices: glium::index::NoIndices,
}

impl Shape {
    pub fn new(display: &Display, id: u32, index_type: PrimitiveType) -> Self {
        let va = VertexArray::new(display);
        let indices = glium::index::NoIndices(index_type);

        Shape { id, va, indices }
    }

    pub fn triangle(display: &Display, id: u32) -> Self {
        let mut temp: Shape = Shape::new(display, id, PrimitiveType::TrianglesList);

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
        temp.va = VertexArray::from_vector(display, vertices);
        temp
    }

}

pub trait Drawable{
    fn draw(&self, target: &mut Frame, program: &Program);
}

impl Drawable for Shape{
    fn draw(&self, target: &mut Frame, program: &Program) {
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ 0.0 , 0.0, 0.0, 1.0f32],
            ]
        };
        target
            .draw(
                &*self.va.vbo,
                &self.indices,
                program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

impl Manipulate for Shape {

    fn rotate(&mut self, angle: f32) {
        self.va.rotate(angle);
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.va.translate(x, y);
    }

    fn move_to_origin(&mut self) {
        self.va.move_to_origin();
    }
}
