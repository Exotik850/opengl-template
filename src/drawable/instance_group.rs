use super::shape::HasShape;
use drawable::{DrawUniforms, Drawable};
use glium::index::NoIndices;
use glium::*;
use rayon::prelude::*;
use std::ops::{Index, IndexMut};
use util::attribute::Attr;
use util::bufferable::{BufferObject, Bufferable};

pub struct InstanceGroup<T>
where
    T: HasShape,
{
    shape: T,
    transforms: BufferObject<Attr>,
}

#[allow(dead_code)]
impl<T> InstanceGroup<T>
where
    T: HasShape,
{
    pub fn new(shape: T, num: usize, display: &Display) -> Self {
        let mut transforms = vec![Attr::default(); num];
        transforms.par_iter_mut().for_each(|p| p.randomize());
        let transforms = Attr::new_vbo(display, &transforms);
        InstanceGroup { shape, transforms }
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

impl<T: HasShape> Drawable for InstanceGroup<T> {
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        uniforms: DrawUniforms,
    ) {
        target
            .draw(
                (self.shape.ref_vbo(), self.transforms.per_instance()),
                &NoIndices(*self.shape.ref_index()),
                &program,
                &uniforms,
                &params,
            )
            .unwrap();
    }
}
