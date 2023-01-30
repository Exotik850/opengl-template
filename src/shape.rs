use crate::vertex::F32vec2;
use glium::index::{NoIndices, PrimitiveType};
use glium::{uniform, Display, Frame, Program, Surface, VertexBuffer};
use winit::event_loop::EventLoop;

// Struct for handling the components of a primitive shape and drawing it to the screen
pub struct Shape {
    pub vertices: Vec<F32vec2>,
    pub vbo: VertexBuffer<F32vec2>,
    pub index_type: PrimitiveType,
    pub id: u32,
}

impl Shape {
    // Default shape
    pub fn triangle(display: &Display) -> Shape {
        let vertex1 = F32vec2 {
            position: [1.0, -1.0],
        } * 0.1;
        let vertex2 = F32vec2 {
            position: [-1.0, -1.0],
        } * 0.1;
        let vertex3 = F32vec2 {
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

// Trait for making sure any other shape made has the same types and handles drawing
pub trait HasShape {
    type RefType;
    fn ref_vertices(&self) -> &Vec<F32vec2>;
    fn mut_vertices(&mut self) -> &mut Vec<F32vec2>;
    fn ref_vbo(&self) -> &VertexBuffer<F32vec2>;
    fn ref_index(&self) -> &PrimitiveType;
    fn get_id(&self) -> u32;
    fn new_vbo(display: &Display, vertices: &Vec<F32vec2>) -> VertexBuffer<F32vec2> {
        VertexBuffer::new(display, &vertices).unwrap()
    }
    fn update_vbo(&self) {
        self.ref_vbo().write(self.ref_vertices())
    }

    fn draw(&self, target: &mut Frame, programs: &Vec<Program>) {
        self.update_vbo();
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ 0.0 , 0.0, 0.0, 1.0f32],
            ],
            world_position: [0.0f32, 0.0]
        };
        target
            .draw(
                self.ref_vbo(),
                &NoIndices(*self.ref_index()),
                &programs[self.get_id() as usize],
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

impl HasShape for Shape {
    type RefType = Shape;
    fn ref_vertices(&self) -> &Vec<F32vec2> {
        &self.vertices
    }
    fn mut_vertices(&mut self) -> &mut Vec<F32vec2> {
        &mut self.vertices
    }
    fn ref_vbo(&self) -> &VertexBuffer<F32vec2> {
        &self.vbo
    }
    fn ref_index(&self) -> &PrimitiveType {
        &self.index_type
    }
    fn get_id(&self) -> u32 {
        self.id
    }
}
