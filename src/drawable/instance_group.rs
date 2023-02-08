use super::shape::HasShape;
use glium::index::{NoIndices, PrimitiveType};
use glium::*;
use rand::Rng;
use rayon::prelude::*;
use std::ops::{Index, IndexMut};
use util::attribute::Attr;
use util::vertex::F32vec3;

pub struct InstanceGroup<T>
where
    T: HasShape,
{
    shape: T,
    transforms: Vec<Attr>,
    transform_buffer: VertexBuffer<Attr>,
}

impl<T> InstanceGroup<T>
where
    T: HasShape,
{
    pub fn new(shape: T, num: usize, display: &Display) -> Self {
        let mut transforms = vec![Attr::default(); num];
        transforms.iter_mut().for_each(|p| p.randomize());

        let transform_buffer = VertexBuffer::dynamic(display, &transforms).unwrap();
        InstanceGroup {
            shape,
            transforms,
            transform_buffer,
        }
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
    fn update_buffers(&self) {
        self.ref_buffer().write(self.ref_data());
        self.ref_shape().update_vbo();
    }
    fn update(&mut self) {}
    fn rotate_z(&mut self, angle: f32);
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

    fn ref_vertices(&self) -> &Vec<F32vec3> {
        self.ref_shape().ref_vertices()
    }
    fn mut_vertices(&mut self) -> &mut Vec<F32vec3> {
        self.mut_shape().mut_vertices()
    }
    fn ref_vbo(&self) -> &VertexBuffer<F32vec3> {
        self.ref_shape().ref_vbo()
    }
    fn ref_index(&self) -> &PrimitiveType {
        self.ref_shape().ref_index()
    }
    fn get_id(&self) -> usize {
        self.ref_shape().get_id()
    }
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        perspective: [[f32; 4]; 4],
    ) {
        self.update_buffers();
        target
            .draw(
                (
                    self.ref_shape().ref_vbo(),
                    self.ref_buffer().per_instance().unwrap(),
                ),
                &NoIndices(*self.ref_shape().ref_index()),
                &program,
                &uniform! {u_light: [-1.0, 0.4, 0.9f32], perspective: perspective},
                &params,
            )
            .unwrap();
    }
}

impl<T> Index<usize> for InstanceGroup<T>
where
    T: HasShape,
{
    type Output = Attr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.transforms[index]
    }
}

impl<T> IndexMut<usize> for InstanceGroup<T>
where
    T: HasShape,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.transforms[index]
    }
}

impl<T> HasPos for InstanceGroup<T>
where
    T: HasShape,
{
    type RefType = T;
    type Type = InstanceGroup<T>;

    fn ref_shape(&self) -> &Self::RefType {
        &self.shape
    }

    fn mut_shape(&mut self) -> &mut Self::RefType {
        &mut self.shape
    }

    fn ref_data(&self) -> &[Attr] {
        self.transforms.as_slice()
    }

    fn mut_data(&mut self) -> &mut [Attr] {
        self.transforms.as_mut_slice()
    }

    fn ref_buffer(&self) -> &VertexBuffer<Attr> {
        &self.transform_buffer
    }

    fn mut_buffer(&mut self) -> &mut VertexBuffer<Attr> {
        &mut self.transform_buffer
    }

    fn update(&mut self) {
        // self.transforms.par_iter_mut().for_each(|p| {});
        // self.transforms.par_iter_mut().for_each(|p| {
        //     let mut v = F32vec3::from(&p.world_position);
        //     // if v.mag() > 4.0 * 2.0f32.sqrt() {
        //     //     panic!("Out of bounds! {:?}", p);
        //     // }
        //     // v.rotate_z(PI);
        //     v *= 0.0001;
        //     p.translate(v.x(), v.y(), v.z());
        // });
    }

    fn rotate_z(&mut self, angle: f32) {
        self.transforms.par_iter_mut().for_each(|p| {
            p.rotate_y(angle);
            p.rotate_z(angle);
        });
    }
}
