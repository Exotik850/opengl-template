use crate::vertex::{Vertex, Manipulate};
use glium::index::{NoIndices, PrimitiveType};
use glium::{uniform, Display, Frame, Program, Surface, VertexBuffer};

pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub vbo: VertexBuffer<Vertex>,
    pub index_type: PrimitiveType,
    pub id: u32,
}

pub trait Drawable<U>{
    fn new(display: &Display, vertices: Vec<Vertex>, index_type: PrimitiveType, id: u32) -> U;
    fn get_id(&self) -> u32;
    fn draw(&self, target: &mut Frame, program: &Program);
    fn clone(&self, display: &Display) -> U;
}

impl Drawable<Shape> for Shape{
    fn new(display: &Display, vertices: Vec<Vertex>, index_type: PrimitiveType, id: u32) -> Shape {
        let vertices = vertices.clone();
        let vbo: VertexBuffer<Vertex> = VertexBuffer::dynamic(display, &vertices).unwrap();

        Shape { vertices, vbo, index_type, id }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

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
                &self.vbo,
                &NoIndices(self.index_type),
                program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    fn clone(&self, display: &Display) -> Shape {
        let vertices = self.vertices.clone();
        Shape::new(display, vertices, self.index_type, self.id)
    }
}

impl Manipulate<Shape> for Shape{
    fn rotate(&mut self, angle: f32) {
        for vertex in self.vertices.iter_mut(){
            vertex.rotate(angle);
        }
        self.vbo.write(&self.vertices)
    }

    fn translate(&mut self, x: f32, y: f32) {
        for vertex in self.vertices.iter_mut(){
            vertex.translate(x, y);
        }
        self.vbo.write(&self.vertices)
    }

    fn move_to_origin(&mut self) {
        for vertex in self.vertices.iter_mut(){
            vertex.move_to_origin();
        }
        self.vbo.write(&self.vertices)
    }
}
