use std::f32::consts::PI;
use glium::index::{PrimitiveType};
use glium::{Display, VertexBuffer};
use shape::{HasShape, Shape};
use vertex::f32Vec2;

pub struct Object
{
    pub world_position: f32Vec2,
    pub rotation: f32,
    pub shape: Shape
}

impl HasPos for Object
{
    type RefType = Shape;
    type Type = Object;
    fn ref_world_pos(&self) -> &f32Vec2 {&self.world_position}
    fn mut_world_pos(&mut self) -> &mut f32Vec2 {&mut self.world_position}
    fn ref_shape(&self) -> &Self::RefType {&self.shape}
    fn rotation(&self) -> f32 {self.rotation}
    fn new(world_position: f32Vec2, rotation: f32, shape: Self::RefType) -> Self::Type {
        Object{world_position, rotation, shape}
    }
}

pub trait HasPos
{
    type RefType: HasShape;
    type Type;
    fn ref_world_pos(&self) -> &f32Vec2;
    fn mut_world_pos(&mut self) -> &mut f32Vec2;
    fn ref_shape(&self) -> &Self::RefType;
    fn rotation(&self) -> f32;
    fn update(&mut self) {}
    fn from_shape(shape: Self::RefType) -> Self::Type {
        let world_position = f32Vec2::new();
        let rotation = 0.0f32;
        Self::new(world_position, rotation, shape)
    }
    fn new(world_position: f32Vec2, rotation: f32, shape: Self::RefType) -> Self::Type;
}

pub trait Manipulate<T>
where
    T: HasShape
{
    fn rotate(&mut self, angle: f32);
    fn set(&mut self, x: f32, y: f32);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl<T, U> Manipulate<T> for T
where
    T: HasPos<RefType=U>,
    U: HasShape
{
    fn rotate(&mut self, angle: f32) {
        let ang = angle * PI / 180.0;
        let tx = ang.cos() * self.x() - ang.sin() * self.y();
        let ty = ang.sin() * self.x() + ang.cos() * self.y();
        self.set(tx, ty);
    }
    fn set(&mut self, x: f32, y: f32)
    {
        self.mut_world_pos().position[0] = x;
        self.mut_world_pos().position[1] = y;
    }

    fn set_x(&mut self, x: f32) { self.mut_world_pos().position[0] = x; }
    fn set_y(&mut self, y: f32) { self.mut_world_pos().position[1] = y;}
    fn x(&self) -> f32 { self.ref_world_pos().position[0] }
    fn y(&self) -> f32 { self.ref_world_pos().position[1] }
}

impl<T: HasPos> HasShape for T
where
    T::RefType: HasShape
{
    type RefType = T::RefType;

    fn ref_vertices(&self) -> &Vec<f32Vec2> { self.ref_shape().ref_vertices() }
    fn ref_vbo(&self) -> &VertexBuffer<f32Vec2> {
        self.ref_shape().ref_vbo()
    }
    fn ref_index(&self) -> &PrimitiveType {
        self.ref_shape().ref_index()
    }
    fn get_id(&self) -> u32 {
        self.ref_shape().get_id()
    }

    fn new(vertices: Vec<f32Vec2>, vbo: VertexBuffer<f32Vec2>, index_type: PrimitiveType, id: u32) -> Self::RefType {
        todo!()
    }
}


// fn draw(&self, target: &mut Frame, program: &Program) {
//     self.ref_shape().update_vbo();
//     let sin = self.rotation().sin();
//     let cos = self.rotation().cos();
//     let uniforms = uniform! {
//         matrix: [
//             [cos * self.x(), -sin * self.y(), 0.0, 0.0],
//             [sin * self.x(), cos * self.y(), 0.0, 0.0],
//             [0.0, 0.0, 1.0, 0.0],
//             [ 0.0 , 0.0, 0.0, 1.0f32],
//         ]
//     };
//     target
//         .draw(
//             self.ref_shape().ref_vbo(),
//             &NoIndices(*self.ref_shape().ref_index()),
//             program,
//             &uniforms,
//             &Default::default(),
//         )
//         .unwrap();
// }