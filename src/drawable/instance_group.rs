use super::shape::HasShape;
use glium::index::{NoIndices};
use glium::*;
use rayon::prelude::*;
use std::ops::{Index, IndexMut};
use util::attribute::Attr;

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
        transforms.par_iter_mut().for_each(|p| p.randomize());
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
    fn ref_shape(&self) -> Box<[&Self::RefType]>;
    fn mut_shape(&mut self) -> Box<[&mut Self::RefType]>;
    fn ref_data(&self) -> &[Attr];
    fn mut_data(&mut self) -> &mut [Attr];
    fn ref_buffer(&self) -> &VertexBuffer<Attr>;
    fn mut_buffer(&mut self) -> &mut VertexBuffer<Attr>;
    fn update_buffers(&self) {
        self.ref_buffer().write(self.ref_data());
        self.ref_shape().iter_mut().for_each(|p| p.update_vbo());
    }
    fn update(&mut self) {}
    fn rotate_z(&mut self, angle: f32);
    fn x(&self) -> f32 {self.ref_data()[0].x()}
    fn y(&self) -> f32 {self.ref_data()[0].y()}
    fn z(&self) -> f32{self.ref_data()[0].z()}
    fn get_id(&self) -> usize {
        self.ref_shape()[0].get_id()
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
                    self.ref_shape()[0].ref_vbo(),
                    self.ref_buffer().per_instance().unwrap(),
                ),
                &NoIndices(*self.ref_shape()[0].ref_index()),
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

    fn ref_shape(&self) -> Box<[&T]> {
        Box::from([&self.shape])
    }

    fn mut_shape(&mut self) -> Box<[&mut T]> {
        Box::from([&mut self.shape])
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

    fn rotate_z(&mut self, angle: f32) {
        self.transforms.par_iter_mut().for_each(|p| {
            p.rotate_y(angle);
            p.rotate_z(angle);
        });
    }
}
