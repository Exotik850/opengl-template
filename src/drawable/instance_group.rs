use glium::{Display, VertexBuffer};
use super::object::HasPos;
use rand::Rng;
use rayon::prelude::*;
use super::shape::HasShape;
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
        transforms.iter_mut().for_each(|p| p.randomize());
        
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