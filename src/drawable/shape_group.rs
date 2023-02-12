use drawable::shape::HasShape;
use drawable::Drawable;
use glium::index::NoIndices;
use glium::{uniform, DrawParameters, Frame, Program, Surface};
use util::attribute::Attr;
use util::bufferable::BufferObject;

pub struct ShapeGroup<T>
where
    T: HasShape,
{
    pub shapes: Vec<T>,
    pub transforms: Vec<BufferObject<Attr>>,
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
        for (shape, transform) in self.shapes.iter().zip(self.transforms.iter()) {
            transform.update_buffer();
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
