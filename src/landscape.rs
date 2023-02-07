use glium::index::PrimitiveType;
use glium::{Display, VertexBuffer};
use noise::{NoiseFn, Perlin};
use object::HasPos;
use shape::Shape;
use std::f32::consts::PI;
use vertex::{Attr, Bufferable, F32vec3};

pub struct Landscape {
    shape: Shape,
    transform: [Attr; 1],
    transform_buffer: VertexBuffer<Attr>,
    noise: Perlin,
    time: f64,
}

impl Landscape {
    pub fn default(display: &Display) -> Self {
        let (cols, rows, res, nres) = (100, 100, 0.05, 0.5);
        let height = 1.0;
        let noise = Perlin::new(0);
        let mut vertices: Vec<F32vec3> = vec![];
        let time = 0.0;
        for i in -cols / 2..cols / 2 {
            for j in -rows / 2..rows / 2 {
                let (x, y) = (i as f64 * res, j as f64 * res);
                let z = noise.get([x * nres, y * nres, time]) * height;
                let z2 = noise.get([x * nres, (y + 1.0) * nres, time]) * height;
                // let z3 = noise.get([(x + 1.0) * nres, y * nres, time]) * height;
                vertices.push(F32vec3::from([x as f32, y as f32, z as f32]));
                vertices.push(F32vec3::from([x as f32, (y + 1.0) as f32, z2 as f32]));
                // vertices.push(F32vec3::from([(x + 1.0) as f32, y as f32, z as f32]));
            }
        }
        let shape = Shape::from_vertices(&vertices, PrimitiveType::LineStrip, display);
        let mut transform = [Attr::default()];
        transform[0].rotate_x(-PI / 3.0);
        let transform_buffer = Attr::new_vbo(display, &transform);
        Landscape {
            shape,
            transform,
            transform_buffer,
            noise,
            time,
        }
    }
}

impl HasPos for Landscape {
    type RefType = Shape;
    type Type = Landscape;

    fn ref_shape(&self) -> &Self::RefType {
        &self.shape
    }

    fn mut_shape(&mut self) -> &mut Self::RefType {
        &mut self.shape
    }

    fn ref_data(&self) -> &[Attr] {
        &self.transform
    }

    fn mut_data(&mut self) -> &mut [Attr] {
        &mut self.transform
    }

    fn ref_buffer(&self) -> &VertexBuffer<Attr> {
        &self.transform_buffer
    }

    fn mut_buffer(&mut self) -> &mut VertexBuffer<Attr> {
        &mut self.transform_buffer
    }

    fn rotateZ(&mut self, angle: f32) {
        self.transform[0].rotate_z(angle);
    }
} //
