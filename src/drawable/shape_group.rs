use drawable::shape::HasShape;
use drawable::Drawable;
use glium::index::NoIndices;
use glium::{uniform, DrawParameters, Frame, Program, Surface};
use rayon::prelude::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge,
    ParallelIterator,
};
use std::iter::{zip, Map};
use std::slice::{Iter, IterMut};
use util::attribute::Attr;
use util::bufferable::BufferObject;

pub struct ShapeGroup<T>
where
    T: HasShape + Send,
{
    pub shapes: Vec<Box<T>>,
    pub transforms: Vec<Box<BufferObject<Attr>>>,
}

impl<T: HasShape + Send> Default for ShapeGroup<T> {
    fn default() -> Self {
        Self {
            shapes: vec![],
            transforms: vec![],
        }
    }
}

impl<T: HasShape + Send> ShapeGroup<T> {
    pub fn push(&mut self, obj: (T, BufferObject<Attr>)) {
        self.shapes.push(Box::from(obj.0));
        self.transforms.push(Box::from(obj.1));
    }

    pub fn iter_shapes(&mut self) -> Iter<'_, Box<T>> {
        self.shapes.iter()
    }

    pub fn iter_mut_shapes(&mut self) -> IterMut<'_, Box<T>> {
        self.shapes.iter_mut()
    }

    pub fn update_buffers(&self) {
        self.transforms.iter().for_each(|p| p.update_buffer());
    }
}

impl<T> Drawable for ShapeGroup<T>
where
    T: HasShape + Send,
{
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        perspective: [[f32; 4]; 4],
    ) {
        self.update_buffers();
        let shapes = self.shapes.as_slice();
        let transforms = self.transforms.as_slice();
        for (shape, transform) in zip(shapes, transforms) {
            target
                .draw(
                    (shape.ref_vbo(), transform.per_instance()),
                    &NoIndices(*shape.ref_index()),
                    &program,
                    &uniform! {u_light: [-1.0, 0.4, 0.9f32], perspective: perspective},
                    &params,
                )
                .unwrap();
        }
    }

    fn rotate_z(&mut self, angle: f32) {
        self.transforms
            .iter_mut()
            .for_each(|t| t.iter_mut().for_each(|t| t.rotate_z(angle)));
    }
}
