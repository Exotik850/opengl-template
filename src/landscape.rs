use drawable::shape::{HasShape, Shape};
use drawable::shape_group::ShapeGroup;
use drawable::{DrawUniforms, Drawable};
use glium::index::PrimitiveType;
use glium::{Display, DrawParameters, Frame, Program};
use noise::{NoiseFn, Perlin};
use rand::{thread_rng, RngCore};
use rayon::prelude::*;
use util::attribute::Attr;
use util::bufferable::Bufferable;
use util::vertex::F32vec3;
use util::Manipulate;

#[derive(Copy, Clone)]
struct Dims(i32, i32, f64, f64);

pub struct Landscape {
    shapes: ShapeGroup<Shape>,
    noise: Perlin,
    time: f64,
    dims: Dims,
}

impl Landscape {
    pub fn default(display: &Display) -> Self {
        let (cols, rows, res, nres) = (100, 100, 0.01, 0.5);
        let height = 1.0;
        let noise = Perlin::new(thread_rng().next_u32());
        let time = 0.0;

        let mut shapes = ShapeGroup::default();
        for i in 0..rows {
            let y = (i as f64 - (rows as f64 / 2.0)) * res;
            let mut vertices = vec![];
            for j in 0..cols {
                let x = (j as f64 - (cols as f64 / 2.0)) * res;
                let z = noise.get([x * nres, y * nres, time]) * height;
                let z2 = noise.get([x * nres, (y + res) * nres, time]) * height;
                vertices.push(F32vec3::from([(x + res) as f32, y as f32, z as f32]));
                vertices.push(F32vec3::from([(x + res) as f32, y as f32, z2 as f32]));
            }
            let shape = Shape::from_vertices(&vertices, PrimitiveType::LineStrip, display);
            let attr = Attr::new_vbo(display, &[Attr::from([0.0, (y * res) as f32, 0.0])]);
            shapes.push((shape, attr));
        }

        Landscape {
            shapes,
            noise,
            time,
            dims: Dims(cols, rows, res, nres),
        }
    }
}

impl Drawable for Landscape {
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        uniforms: DrawUniforms,
    ) {
        self.shapes.draw(target, program, params, uniforms);
    }

    fn update(&mut self) {
        let dims = self.dims.clone();
        let time = self.time;
        let noise = self.noise;
        let shapes = self.shapes.iter_mut_shapes().enumerate();

        shapes.for_each(|(p, shape)| {
            for (idx, verts) in shape.vertices.mut_data().chunks_mut(2).enumerate() {
                let x = idx as f64 * dims.2;
                let y = p as f64 * dims.2;
                verts[0].position[1] = noise.get([x, y, time]) as f32;
                verts[1].position[1] = noise.get([x, y + dims.2 * dims.3, time]) as f32;
            }
            shape.vertices.update_buffer();
        });
        self.time += 0.01;
    }
}

impl Manipulate for Landscape {
    fn rotate_axis(&mut self, axis: usize, ang: f32) {
        self.shapes.rotate_axis(axis, ang);
    }
}
