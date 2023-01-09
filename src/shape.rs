use crate::vertex::{f32Vec2};
use glium::index::{NoIndices, PrimitiveType};
use glium::{uniform, Display, Frame, Program, Surface, VertexBuffer};

pub struct Shape
{
    pub vertices: Vec<f32Vec2>,
    pub vbo: VertexBuffer<f32Vec2>,
    pub index_type: PrimitiveType,
    pub id: u32,
}

pub trait HasShape
{
    type RefType;
    fn ref_vertices(&self) -> &Vec<f32Vec2>;
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
            ]
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
    fn ref_vbo(&self) -> &VertexBuffer<f32Vec2> { &self.vbo }
    fn ref_index(&self) -> &PrimitiveType { &self.index_type }
    fn get_id(&self) -> u32 { self.id }
    fn new(vertices: Vec<f32Vec2>, vbo: VertexBuffer<f32Vec2>, index_type: PrimitiveType, id: u32) -> Self::RefType {
        Shape{ vertices, vbo, index_type, id, }
    }
}