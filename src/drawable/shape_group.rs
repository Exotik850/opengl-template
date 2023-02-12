use drawable::shape::HasShape;
use drawable::Drawable;
use glium::index::NoIndices;
use glium::{uniform, DrawParameters, Frame, Program, Surface};
use std::slice::{Iter, IterMut};
use util::attribute::Attr;
use util::bufferable::BufferObject;

pub struct ShapeGroup<T>
where
    T: HasShape,
{
    pub shapes: Vec<T>,
    pub transforms: Vec<BufferObject<Attr>>,
}

impl<T: HasShape> Default for ShapeGroup<T> {
    fn default() -> Self {
        Self {
            shapes: vec![],
            transforms: vec![],
        }
    }
}

impl<T: HasShape> ShapeGroup<T> {
    pub fn push(&mut self, obj: (T, BufferObject<Attr>)) {
        self.shapes.push(obj.0);
        self.transforms.push(obj.1);
    }

    pub fn iter_shapes(&mut self) -> Iter<'_, T> {
        self.shapes.iter()
    }

    pub fn iter_mut_shapes(&mut self) -> IterMut<'_, T> {
        self.shapes.iter_mut()
    }

    pub fn update_buffers(&self) {
        self.transforms.iter().for_each(|p| p.update_buffer());
    }
}

impl<T> Drawable for ShapeGroup<T>
where
    T: HasShape,
{
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        perspective: [[f32; 4]; 4],
    ) {
        self.update_buffers();
        for (shape, transform) in self.shapes.iter().zip(self.transforms.iter()) {
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
