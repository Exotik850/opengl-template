use std::iter::zip;
use drawable::shape::HasShape;
use drawable::{DrawUniforms, Drawable};
use glium::index::NoIndices;
use glium::{uniform, DrawParameters, Frame, Program, Surface};
use std::slice::{Iter, IterMut};
use util::attribute::Attr;
use util::bufferable::BufferObject;
use util::Manipulate;

///
/// Shape groups: Main class 
/// Can hold multiple shapes and multiple attribute buffers for 
/// better instancing control. Add shapes and BufferObjects before use
/// 

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
    
    pub fn iter_transforms(&self, index: usize) -> Iter<'_, Attr> {
        assert!(index < self.shapes.len());
        self.transforms[index].iter()
    }

    pub fn iter_mut_transforms(&mut self, index: usize) -> IterMut<'_, Attr> {
        assert!(index < self.shapes.len());
        self.transforms[index].iter_mut()
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
        uniforms: DrawUniforms,
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
                    &uniforms,
                    &params,
                )
                .unwrap();
        }
    }
}

impl<T: HasShape + Send> Manipulate for ShapeGroup<T> {
    fn rotate_axis(&mut self, axis: usize, ang: f32) {
        self.transforms
            .iter_mut()
            .for_each(|p| p.rotate_axis(axis, ang))
    }
}
