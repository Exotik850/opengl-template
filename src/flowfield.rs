use glium::index::PrimitiveType;
use glium::Display;
use noise::{NoiseFn, Perlin};
use object::HasPos;
use rand::{thread_rng, Rng};
use shape::{HasShape, Shape};
use std::f64::consts::PI;
use std::iter::zip;
use vertex::F32vec3;

// Unused code as of now

pub struct FlowField {
    grid: Vec<F32vec3>,
    velocities: Vec<F32vec3>,
    cols: u32,
    rows: u32,
    res: u32,
    noise: Perlin,
    shape: Shape,
    world_position: F32vec3,
    rotation: f32,
}

impl FlowField {
    pub fn init(display: &Display, res: u32) -> FlowField {
        let width = display.gl_window().window().inner_size().width;
        let height = display.gl_window().window().inner_size().height;
        let cols = width / res;
        let rows = height / res;
        let noise = Perlin::new(0);
        let mut grid = vec![];
        for i in 0..cols {
            for j in 0..rows {
                let ang = noise.get([i as f64 * 0.01, j as f64 * 0.01, 0.0]) * PI * 2.0;
                let tx = ang.cos() as f32;
                let ty = ang.sin() as f32;
                grid.push(F32vec3::from([tx, ty, 0.0]) * 0.0001);
            }
        }
        let num_parts = 1000;
        let mut vertices = vec![];
        let mut velocities = vec![];
        let mut rand = thread_rng();
        for _ in 0..num_parts {
            let x: f32 = rand.gen_range(-2.0..2.0);
            let y: f32 = rand.gen_range(-2.0..2.0);
            vertices.push(F32vec3::from([x / 2.0, y / 2.0, 0.0]));
            vertices.push(F32vec3::from([x, y, 0.0]));
            let ang: f32 = rand.gen();
            let tx = ang.cos();
            let ty = ang.sin();
            velocities.push(F32vec3::from([tx, ty, 0.0]) * 0.0001);
        }
        FlowField {
            grid,
            velocities,
            cols,
            rows,
            res,
            noise,
            shape: Shape {
                vertices: vertices.clone(),
                vbo: Shape::new_vbo(display, &vertices),
                index_type: PrimitiveType::LinesList,
                id: 0,
            },
            world_position: F32vec3::from([0.0; 3]),
            rotation: 0.0,
        }
    }

    fn update_previous_positions(particles: &mut Vec<F32vec3>) {
        for i in 0..particles.len() / 2 {
            let previous_index = i * 2;
            let current_index = previous_index + 1;
            let current_position = particles[current_index].position;
            particles[previous_index].position = current_position;
        }
    }
}

// impl HasPos for FlowField {
//     type RefType = Shape;
//     type Type = FlowField;
//
//     fn ref_world_pos(&self) -> &F32vec2 {
//         &self.world_position
//     }
//
//     fn mut_world_pos(&mut self) -> &mut F32vec2 {
//         &mut self.world_position
//     }
//
//     fn ref_shape(&self) -> &Self::RefType {
//         &self.shape
//     }
//
//     fn mut_shape(&mut self) -> &mut Self::RefType {
//         &mut self.shape
//     }
//
//     fn rotation(&self) -> f32 {
//         self.rotation
//     }
//
//     fn update(&mut self) {
//         let res = self.res;
//         let cols = self.cols;
//         let grid = self.grid.clone();
//         for (i, pos) in self.mut_vertices().iter_mut().enumerate() {
//             if i % 2 != 0 {
//                 continue;
//             }
//             let x = pos.x() as u32 / res;
//             let y = pos.y() as u32 / res;
//             let index = y * cols + x;
//             *pos += grid[index as usize];
//         }
//         FlowField::update_previous_positions(self.mut_vertices());
//     }
//
//     fn rotate(&mut self, angle: f32) {
//         self.rotation += angle;
//     }
// }
