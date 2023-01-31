use glium::{Display, VertexBuffer};
use object::HasPos;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use shape::HasShape;
use std::ops::{Index, IndexMut};
use vertex::{Attr, F32vec2};

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
        for transform in &mut transforms {
            transform.rand()
        }
        let transform_buffer = VertexBuffer::dynamic(display, &transforms).unwrap();
        InstanceGroup {
            shape,
            transforms,
            transform_buffer,
        }
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
        &self.transforms[..]
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
    }

    fn rotate(&mut self, angle: f32) {
        self.transforms.par_iter_mut().for_each(|p| p.rotateZ(angle));
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        todo!()
    }
}
