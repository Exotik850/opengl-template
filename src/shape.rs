use crate::vertex::f32Vec2;
use glium::index::{NoIndices, PrimitiveType};
use glium::{uniform, Display, Frame, Program, Surface, VertexBuffer};
use winit::event_loop::EventLoop;

pub struct Shape {
    pub vertices: Vec<f32Vec2>,
    pub vbo: VertexBuffer<f32Vec2>,
    pub index_type: PrimitiveType,
    pub id: u32,
}

impl Shape {
    pub fn triangle(display: &Display) -> Shape {
        let vertex1 = f32Vec2 {
            position: [1.0, -1.0],
        } * 0.1;
        let vertex2 = f32Vec2 {
            position: [-1.0, -1.0],
        } * 0.1;
        let vertex3 = f32Vec2 {
            position: [0.0, 1.0],
        } * 0.1;
        let vertices = vec![vertex1, vertex2, vertex3];
        Shape {
            vertices: vertices.clone(),
            vbo: Shape::new_vbo(&display, &vertices),
            index_type: PrimitiveType::TrianglesList,
            id: 0,
        }
    }
}

pub trait HasShape {
    type RefType;
    fn ref_vertices(&self) -> &Vec<f32Vec2>;
    fn mut_vertices(&mut self) -> &mut Vec<f32Vec2>;
    fn ref_vbo(&self) -> &VertexBuffer<f32Vec2>;
    fn ref_index(&self) -> &PrimitiveType;
    fn get_id(&self) -> u32;
    fn new_vbo(display: &Display, vertices: &Vec<f32Vec2>) -> VertexBuffer<f32Vec2> {
        VertexBuffer::new(display, &vertices).unwrap()
    }
    fn update_vbo(&self) {
        self.ref_vbo().write(self.ref_vertices())
    }

    fn draw(&self, target: &mut Frame, program: &Program) {
        self.update_vbo();
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ 0.0 , 0.0, 0.0, 1.0f32],
            ],
            world_position: [0.0, 0.0]
        };
        target
            .draw(
                self.ref_vbo(),
                &NoIndices(*self.ref_index()),
                program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

impl HasShape for Shape {
    type RefType = Shape;
    fn ref_vertices(&self) -> &Vec<f32Vec2> {
        &self.vertices
    }
    fn mut_vertices(&mut self) -> &mut Vec<f32Vec2> {
        &mut self.vertices
    }
    fn ref_vbo(&self) -> &VertexBuffer<f32Vec2> {
        &self.vbo
    }
    fn ref_index(&self) -> &PrimitiveType {
        &self.index_type
    }
    fn get_id(&self) -> u32 {
        self.id
    }
}
