use drawable::instance_group::HasPos;
use drawable::shape::HasShape;
use glium::VertexBuffer;
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

    fn ref_shape(&self) -> &Self::RefType {
        todo!()
    }

    fn mut_shape(&mut self) -> &mut Self::RefType {
        todo!()
    }

    fn ref_data(&self) -> &[Attr] {
        todo!()
    }

    fn mut_data(&mut self) -> &mut [Attr] {
        todo!()
    }

    fn ref_buffer(&self) -> &VertexBuffer<Attr> {
        todo!()
    }

    fn mut_buffer(&mut self) -> &mut VertexBuffer<Attr> {
        todo!()
    }

    fn rotate_z(&mut self, angle: f32) {
        todo!()
    }
}
