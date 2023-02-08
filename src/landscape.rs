use drawable::instance_group::HasPos;
use drawable::shape::Shape;
use glium::index::PrimitiveType;
use glium::{Display, VertexBuffer};
use noise::{NoiseFn, Perlin};
use rand::{thread_rng, RngCore};
use rayon::prelude::*;
use std::f32::consts::PI;
use util::attribute::Attr;
use util::bufferable::Bufferable;
use util::vertex::F32vec3;

#[derive(Copy, Clone)]
struct Dims(i32, i32, f64, f64);

pub struct Landscape {
    shape: Shape,
    transform: [Attr; 1],
    transform_buffer: VertexBuffer<Attr>,
    noise: Perlin,
    time: f64,
    dims: Dims,
}

impl Landscape {
    pub fn default(display: &Display) -> Self {
        let (cols, rows, res, nres) = (100, 100, 0.01, 1.0);
        let height = 1.0;
        let noise = Perlin::new(thread_rng().next_u32());
        let time = 0.0;

        let vertices: Vec<F32vec3> = (0..rows)
            .into_par_iter()
            .flat_map(|j| {
                let y = j as f64 * res - (rows as f64 / 2.0) * res;
                (0..cols)
                    .into_iter()
                    .map(move |i| {
                        let x = i as f64 * res - (cols as f64 / 2.0) * res;
                        let z = noise.get([x * nres, y * nres, time]) * height;
                        let z2 = noise.get([x * nres, (y + res) * nres, time]) * height;
                        vec![
                            F32vec3::from([x as f32, y as f32, z as f32]),
                            F32vec3::from([x as f32, (y + res) as f32, z2 as f32]),
                        ]
                    })
                    .flatten()
                    .collect::<Vec<F32vec3>>()
            })
            .collect();

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
            dims: Dims(cols, rows, res, nres),
        }
    }
}

impl HasPos for Landscape {
    type RefType = Shape;
    type Type = Landscape;

    fn ref_shape(&self) -> Box<[&Shape]> {
        Box::from([&self.shape])
    }

    fn mut_shape(&mut self) -> Box<[&mut Shape]> {
        Box::from([&mut self.shape])
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

    fn update(&mut self) {
        let dims = self.dims.clone();
        let time = self.time;
        let noise = self.noise;
        self.shape
            .vertices
            .par_iter_mut()
            .chunks(2)
            .enumerate()
            .for_each(|(idx, mut verts)| {
                let x = ((idx % dims.0 as usize) as f64 + 1.0) * dims.2;
                let y = ((idx / dims.0 as usize) as f64 + 1.0) * dims.2;
                verts[0].position[2] = noise.get([x, y, time]) as f32;
                verts[1].position[2] = noise.get([x, y + dims.2 * dims.3, time]) as f32;
            });
        self.time += 0.01;
    }

    fn rotate_z(&mut self, angle: f32) {
        self.transform[0].rotate_z(angle);
    }
}
