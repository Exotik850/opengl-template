use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct F32vec2 {
    pub position: [f32; 2],
}
glium::implement_vertex!(F32vec2, position);

#[derive(Copy, Clone, Debug)]
pub struct Attr {
    pub world_position: [f32; 2],
    pub rotation_matrix: [[f32; 4]; 4],
}
glium::implement_vertex!(Attr, world_position, rotation_matrix);

impl Default for Attr {
    fn default() -> Self {
        let world_position = [0.0, 0.0];
        let rotation_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Attr {
            world_position,
            rotation_matrix,
        }
    }
}

impl Attr {
    pub fn translate(&mut self, x: f32, y: f32) {
        self.world_position[0] += x;
        self.world_position[1] += y;
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.world_position[0] = x;
        self.world_position[1] = y;
    }

    pub fn x(&self) -> f32 {
        self.world_position[0]
    }

    pub fn y(&self) -> f32 {
        self.world_position[1]
    }

    pub fn rotate(&mut self, ang: f32) {
        let cos_theta = ang.cos();
        let sin_theta = ang.sin();
        for i in 0..4 {
            let x = self.rotation_matrix[i][0];
            let y = self.rotation_matrix[i][1];
            self.rotation_matrix[i][0] = cos_theta * x - sin_theta * y;
            self.rotation_matrix[i][1] = sin_theta * x + cos_theta * y;
        }
    }

    pub fn rand(&mut self) {
        self.world_position[0] = thread_rng().gen_range(-2.0..2.0);
        self.world_position[1] = thread_rng().gen_range(-2.0..2.0);
    }
}

#[allow(dead_code)]
impl F32vec2 {
    pub fn new() -> Self {
        F32vec2 { position: [0.0; 2] }
    }
    pub fn x(&self) -> f32 {
        self.position[0]
    }
    pub fn y(&self) -> f32 {
        self.position[1]
    }
    pub fn mag_sq(&self) -> f32 {
        self.position[0] * self.position[0] + self.position[1] * self.position[1]
    }
    pub fn mag(&self) -> f32 {
        self.mag_sq().sqrt()
    }
    pub fn normalize(&mut self) {
        *self /= self.mag();
    }
    pub fn limit(&mut self, limit: f32) {
        if self.mag_sq() < limit.powi(2) {
            return;
        }
        self.normalize();
        *self *= limit;
    }
}

impl ops::Add for F32vec2 {
    type Output = F32vec2;
    fn add(self, other: Self) -> Self::Output {
        F32vec2 {
            position: [
                self.position[0] + other.position[0],
                self.position[1] + other.position[1],
            ],
        }
    }
}

impl ops::AddAssign for F32vec2 {
    fn add_assign(&mut self, other: Self) {
        self.position[0] += other.position[0];
        self.position[1] += other.position[1];
    }
}

impl ops::Sub for F32vec2 {
    type Output = F32vec2;
    fn sub(self, other: Self) -> Self::Output {
        F32vec2 {
            position: [
                self.position[0] - other.position[0],
                self.position[1] - other.position[1],
            ],
        }
    }
}

impl ops::SubAssign for F32vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.position[0] -= other.position[0];
        self.position[1] -= other.position[1];
    }
}

impl ops::Mul<f32> for F32vec2 {
    type Output = F32vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        F32vec2 {
            position: [self.position[0] * rhs, self.position[1] * rhs],
        }
    }
}

impl ops::MulAssign<f32> for F32vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.position[0] *= rhs;
        self.position[1] *= rhs;
    }
}

impl ops::Div<f32> for F32vec2 {
    type Output = F32vec2;
    fn div(self, rhs: f32) -> Self::Output {
        F32vec2 {
            position: [self.position[0] / rhs, self.position[1] / rhs],
        }
    }
}

impl ops::DivAssign<f32> for F32vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.position[0] /= rhs;
        self.position[1] /= rhs;
    }
}

impl Display for F32vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.position[0], self.position[1])
    }
}
