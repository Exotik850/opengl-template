use drawable::instance_group::HasPos;
use drawable::shape::HasShape;
use glium::{DrawParameters, Frame, Program, Surface, uniform, VertexBuffer};
use glium::index::NoIndices;
use rayon::prelude::*;
use util::attribute::Attr;

struct ShapeGroup<T>
where
    T: HasShape,
{
    shapes: Vec<T>,
    transforms: Vec<Attr>,
    transform_buffer: VertexBuffer<Attr>,
    ids: Vec<usize>,
}

impl<T> HasPos for ShapeGroup<T>
where
    T: HasShape,
{
    type RefType = T;
    type Type = ShapeGroup<T>;

    fn ref_shape(&self) -> Box<[&Self::RefType]> {
        todo!()
    }

    fn mut_shape(&mut self) -> Box<[&mut Self::RefType]> {
        todo!()
    }

    fn ref_data(&self) -> &[Attr] {
        &self.transforms
    }

    fn mut_data(&mut self) -> &mut [Attr] {
        &mut self.transforms
    }

    fn ref_buffer(&self) -> &VertexBuffer<Attr> {
        &self.transform_buffer
    }

    fn mut_buffer(&mut self) -> &mut VertexBuffer<Attr> {
        &mut self.transform_buffer
    }

    fn rotate_z(&mut self, angle: f32) {
        self.transforms.par_iter_mut().for_each(|t| t.rotate_z(angle));
    }

    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        perspective: [[f32; 4]; 4],
    ) {
        self.update_buffers();
        // for &i in self.ids.iter().zip() {
        //
        // }
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
