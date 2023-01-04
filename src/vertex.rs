use glium::{Display, VertexBuffer};
use std::f32::consts::{PI};
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

#[allow(dead_code)]
impl Vertex {
    pub fn new() -> Self {
        Vertex { position: [0.0; 2] }
    }
}

pub trait Manipulate<T>
{
    fn rotate(&mut self, angle: f32);
    fn translate(&mut self, x: f32, y: f32);
    fn move_to_origin(&mut self);
}

impl Manipulate<Vertex> for Vertex{
    fn rotate(&mut self, angle: f32) {
        let ang = angle * PI / 180.0;
        let tx = ang.cos() * self.position[0] - ang.sin() * self.position[1];
        let ty = ang.sin() * self.position[0] + ang.cos() * self.position[1];
        self.position = [tx, ty];
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    fn move_to_origin(&mut self) {
        self.position = [0.0, 0.0];
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
        self.position[1] *= rhs;
    }
}

impl ops::Div<f32> for Vertex {
    type Output = Vertex;
    fn div(self, rhs: f32) -> Self::Output {
        Vertex {
            position: [self.position[0] / rhs, self.position[1] / rhs],
        }
    }
}

impl ops::DivAssign<f32> for Vertex {
    fn div_assign(&mut self, rhs: f32) {
        self.position[0] /= rhs;
        self.position[1] /= rhs;
    }
}
