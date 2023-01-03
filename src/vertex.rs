use glium::{Display, VertexBuffer};
use std::f32::consts::TAU;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

#[allow(dead_code)]
impl Vertex {
    fn new() -> Self {
        Vertex { position: [0.0; 2] }
    }
}

#[derive(Debug)]
pub struct VertexArray {
    pub vertices: Vec<Vertex>,
    pub vbo: VertexBuffer<Vertex>,
}

impl VertexArray {
    pub fn new(display: &Display) -> Self {
        let vertices: Vec<Vertex> = vec![];
        let vbo = VertexBuffer::dynamic(display, &vertices).unwrap();
        VertexArray { vertices, vbo }
    }

    pub fn from_vector(display: &Display, vertices: Vec<Vertex>) -> Self {
        let vbo = VertexBuffer::dynamic(display, &vertices).unwrap();
        VertexArray { vertices, vbo }
    }
}

impl ops::Add for Vertex {
    type Output = Vertex;
    fn add(self, other: Self) -> Self::Output {
        Vertex {
            position: [
                self.position[0] + other.position[0],
                self.position[1] + other.position[1],
            ],
        }
    }
}

impl ops::AddAssign for Vertex {
    fn add_assign(&mut self, other: Self) {
        self.position[0] += other.position[0];
        self.position[1] += other.position[1];
    }
}

impl ops::Sub for Vertex {
    type Output = Vertex;
    fn sub(self, other: Self) -> Self::Output {
        Vertex {
            position: [
                self.position[0] - other.position[0],
                self.position[1] - other.position[1],
            ],
        }
    }
}

impl ops::SubAssign for Vertex {
    fn sub_assign(&mut self, other: Self) {
        self.position[0] -= other.position[0];
        self.position[1] -= other.position[1];
    }
}

impl ops::Mul<f32> for Vertex {
    type Output = Vertex;
    fn mul(self, rhs: f32) -> Self::Output {
        Vertex {
            position: [self.position[0] * rhs, self.position[1] * rhs],
        }
    }
}

impl ops::MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position[0] *= rhs;
    }
}

impl ops::Div<f32> for Vertex {
    type Output = Vertex;
    fn div(self, rhs: f32) -> Self::Output {
        Vertex {
            position: [self.position[0] * rhs, self.position[1] * rhs],
        }
    }
}

impl ops::DivAssign<f32> for Vertex {
    fn div_assign(&mut self, rhs: f32) {
        self.position[0] *= rhs;
    }
}

pub trait Manipulate {
    fn rotate(&mut self, ang: f32);
    fn translate(&mut self, x: f32, y: f32);
    fn move_to_origin(&mut self);
}

impl Manipulate for Vertex {
    fn rotate(&mut self, ang: f32) {
        let ang = ang * TAU / 180.0;
        let tx = ang.cos() * self.position[0] - ang.sin() * self.position[1];
        let ty = ang.sin() * self.position[0] + ang.cos() * self.position[1];
        self.position[0] = tx;
        self.position[1] = ty;
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.position[0] += x;
        self.position[1] += y;
    }

    fn move_to_origin(&mut self) {
        self.position = [0.0; 2];
    }
}

impl Manipulate for VertexArray {
    fn rotate(&mut self, angle: f32) {
        for s in self.vertices.iter_mut() {
            s.rotate(angle);
        }
        self.vbo.write(&self.vertices)
    }

    fn translate(&mut self, x: f32, y: f32) {
        for s in self.vertices.iter_mut() {
            s.translate(x, y);
        }
        self.vbo.write(&self.vertices);
    }

    fn move_to_origin(&mut self) {
        for s in self.vertices.iter_mut() {
            s.move_to_origin();
        }
        self.vbo.write(&self.vertices);
    }
}
