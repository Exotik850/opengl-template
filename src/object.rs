use glium::index::{NoIndices, PrimitiveType};
use glium::program::Uniform;
use glium::uniforms::UniformBuffer;
use glium::{uniform, Display, Frame, Program, Surface, VertexBuffer};
use shape::{HasShape, Shape};
use vertex::{F32vec2, Attr};

pub struct Object {
    pub shape: Shape,
    pub transform: [Attr; 1],
    pub transform_buffer: VertexBuffer<Attr>,
}

impl Object {
    pub fn from_shape(shape: Shape, display: &Display) -> Object {
        let transform = [Attr::default()];
        let transform_buffer = VertexBuffer::dynamic(display, &transform).unwrap();
        Object {
            shape,
            transform,
            transform_buffer,
        }
    }
}
impl HasPos for Object {
    type RefType = Shape;
    type Type = Object;
    fn ref_shape(&self) -> &Self::RefType {
        &self.shape
    }
    fn mut_shape(&mut self) -> &mut Self::RefType {
        &mut self.shape
    }

    fn ref_data(&self) -> &[Attr] {
        &self.transform
    }

    fn mut_data(&mut self) -> &mut [Attr] {
        &mut self.transform
    }

    fn ref_buffer(&self) -> &VertexBuffer<Attr> {
        &self.transform_buffer
    }

    fn mut_buffer(&mut self) -> &mut VertexBuffer<Attr> {
        &mut self.transform_buffer
    }

    fn update_buffer(&mut self) {
        self.transform_buffer.write(&self.transform);
    }

    fn rotate(&mut self, angle: f32) {
        self.transform[0].rotate(angle);
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.transform[0].set(x, y);
    }
}

pub trait HasPos {
    type RefType: HasShape;
    type Type;
    fn ref_shape(&self) -> &Self::RefType;
    fn mut_shape(&mut self) -> &mut Self::RefType;
    fn ref_data(&self) -> &[Attr];
    fn mut_data(&mut self) -> &mut [Attr];
    fn ref_buffer(&self) -> &VertexBuffer<Attr>;
    fn mut_buffer(&mut self) -> &mut VertexBuffer<Attr>;
    fn update_buffer(&mut self);
    fn update(&mut self) {}
    fn rotate(&mut self, angle: f32);
    fn set_pos(&mut self, x: f32, y: f32);
    fn x(&self) -> f32 {
        self.ref_data()[0].x()
    }
    fn y(&self) -> f32 {
        self.ref_data()[0].y()
    }
}

impl<T: HasPos> HasShape for T
where
    T::RefType: HasShape,
{
    type RefType = T::RefType;

    fn ref_vertices(&self) -> &Vec<F32vec2> {
        self.ref_shape().ref_vertices()
    }
    fn mut_vertices(&mut self) -> &mut Vec<F32vec2> {
        self.mut_shape().mut_vertices()
    }
    fn ref_vbo(&self) -> &VertexBuffer<F32vec2> {
        self.ref_shape().ref_vbo()
    }
    fn ref_index(&self) -> &PrimitiveType {
        self.ref_shape().ref_index()
    }
    fn get_id(&self) -> u32 {
        self.ref_shape().get_id()
    }

    fn draw(&self, target: &mut Frame, program: &Vec<Program>) {
        self.ref_shape().update_vbo();
        self.ref_buffer().write(self.ref_data());
        target
            .draw(
                (
                    self.ref_shape().ref_vbo(),
                    self.ref_buffer().per_instance().unwrap(),
                ),
                &NoIndices(*self.ref_shape().ref_index()),
                &program[self.get_id() as usize],
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
