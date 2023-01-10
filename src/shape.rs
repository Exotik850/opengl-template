use crate::vertex::{f32Vec2};
use glium::index::{NoIndices, PrimitiveType};
use glium::{uniform, Display, Frame, Program, Surface, VertexBuffer};
use winit::event_loop::EventLoop;

pub struct Shape
{
    pub vertices: Vec<f32Vec2>,
    pub vbo: VertexBuffer<f32Vec2>,
    pub index_type: PrimitiveType,
    pub id: u32,
}

impl Shape{
    fn triangle(display: &Display) -> Shape{
        let vertex1 = f32Vec2{ position: [1.0, -1.0] };
        let vertex2 = f32Vec2{ position: [-1.0, -1.0]};
        let vertex3 = f32Vec2{ position: [0.0, 1.0]};
        let vertices = vec![vertex1, vertex2, vertex3];
        let shape = Shape{
            vertices: vertices.clone(),
            vbo: Shape::new_vbo(&display, &vertices),
            index_type: PrimitiveType::TrianglesList,
            id: 0,
        };
        shape
    }
}

pub trait HasShape
{
    type RefType;
    fn ref_vertices(&self) -> &Vec<f32Vec2>;
    fn mut_vertices(&mut self) -> &mut Vec<f32Vec2>;
    fn ref_vbo(&self) -> &VertexBuffer<f32Vec2>;
    fn ref_index(&self) -> &PrimitiveType;
    fn get_id(&self) -> u32;
    fn new_vbo(display: &Display, vertices: &Vec<f32Vec2>) -> VertexBuffer<f32Vec2>
    {
        VertexBuffer::new(display, &vertices).unwrap()
    }
    fn update_vbo(&self) {self.ref_vbo().write(self.ref_vertices())}
    fn new(vertices: Vec<f32Vec2>, vbo:VertexBuffer<f32Vec2>, index_type: PrimitiveType, id: u32) -> Self::RefType;
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

impl HasShape for Shape
{
    type RefType = Shape;
    fn ref_vertices(&self) -> &Vec<f32Vec2> { &self.vertices }
    fn mut_vertices(&mut self) -> &mut Vec<f32Vec2> {&mut self.vertices}
    fn ref_vbo(&self) -> &VertexBuffer<f32Vec2> { &self.vbo }
    fn ref_index(&self) -> &PrimitiveType { &self.index_type }
    fn get_id(&self) -> u32 { self.id }
    fn new(vertices: Vec<f32Vec2>, vbo: VertexBuffer<f32Vec2>, index_type: PrimitiveType, id: u32) -> Self::RefType {
        Shape{ vertices, vbo, index_type, id, }
    }
}