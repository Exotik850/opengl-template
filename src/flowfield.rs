use std::f64::consts::PI;
use std::ops::Index;
use glium::Display;
use glium::index::PrimitiveType;
use object::HasPos;
use shape::{HasShape, Shape};
use vertex::f32Vec2;
use noise::{NoiseFn, Perlin};
use rand::{Rng, thread_rng};

pub struct FlowField
{
    grid: Vec<f32Vec2>,
    velocities: Vec<f32Vec2>,
    cols: u32,
    rows: u32,
    res: u32,
    noise: Perlin,
    shape: Shape,
    world_position: f32Vec2,
    rotation: f32,
}

impl FlowField
{
    pub fn init(display: &Display, res: u32) -> FlowField{
        let width = display.gl_window().window().inner_size().width;
        let height = display.gl_window().window().inner_size().height;
        let cols = width / res;
        let rows = height / res;
        let noise = Perlin::new(0);
        let mut grid = vec![];
        for i in 0..cols{
            for j in 0..rows{
                let ang = noise.get([i as f64 * 0.01, j as f64 * 0.01, 0.0]) * PI * 2.0;
                let tx = ang.cos() as f32;
                let ty = ang.sin() as f32;
                grid.push(f32Vec2{position:[tx, ty]});

            }
        }
        let num_parts = 1000;
        let mut vertices = vec![];
        let mut velocities = vec![];
        let mut rand = thread_rng();
        for _ in 0..num_parts{
            let x: f32 = rand.gen();
            let y: f32 = rand.gen();
            vertices.push(f32Vec2{position:[x, y]});
            vertices.push(f32Vec2{position:[x, y]});
            let ang: f32 = rand.gen();
            let tx = ang.cos();
            let ty = ang.sin();
            velocities.push(f32Vec2{position:[tx, ty]});
        }
        FlowField{
            grid,
            velocities,
            cols,
            rows,
            res,
            noise,
            shape: Shape {
                vertices: vertices.clone(),
                vbo: Shape::new_vbo(display, &vertices),
                index_type: PrimitiveType::LinesListAdjacency,
                id: 0
            },
            world_position: f32Vec2 {position:[0.0, 0.0]},
            rotation: 0.0,
        }
    }
}

impl HasPos for FlowField
{
    type RefType = Shape;
    type Type = FlowField;

    fn ref_world_pos(&self) -> &f32Vec2 {
        &self.world_position
    }

    fn mut_world_pos(&mut self) -> &mut f32Vec2 {
        &mut self.world_position
    }

    fn ref_shape(&self) -> &Self::RefType {
        &self.shape
    }

    fn mut_shape(&mut self) -> &mut Self::RefType { &mut self.shape }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn update(&mut self) {
        for mut i in 0..self.velocities.len()-1{
            self.mut_vertices()[i] = self.mut_vertices()[i + 1];
            let x = self.mut_vertices()[i + 1].position[0] as u32 / self.res;
            let y = self.mut_vertices()[i + 1].position[1] as u32 / self.res;
            let index = y * self.cols + x;
            self.velocities[i] += self.grid[index as usize];
            let after = self.velocities[i];
            self.mut_vertices()[i + 1] += after;

            i += 2;
        }
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }
}