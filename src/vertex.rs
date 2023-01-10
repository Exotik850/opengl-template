use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct f32Vec2 {
    pub position: [f32; 2],
}
glium::implement_vertex!(f32Vec2, position);

#[allow(dead_code)]
impl f32Vec2 {
    pub fn new() -> Self {
        f32Vec2 { position: [0.0; 2] }
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
        if self.mag() < limit {
            return;
        }
        self.normalize();
        *self *= limit;
    }
}

impl ops::Add for f32Vec2 {
    type Output = f32Vec2;
    fn add(self, other: Self) -> Self::Output {
        f32Vec2 {
            position: [
                self.position[0] + other.position[0],
                self.position[1] + other.position[1],
            ],
        }
    }
}

impl ops::AddAssign for f32Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.position[0] += other.position[0];
        self.position[1] += other.position[1];
    }
}

impl ops::Sub for f32Vec2 {
    type Output = f32Vec2;
    fn sub(self, other: Self) -> Self::Output {
        f32Vec2 {
            position: [
                self.position[0] - other.position[0],
                self.position[1] - other.position[1],
            ],
        }
    }
}

impl ops::SubAssign for f32Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.position[0] -= other.position[0];
        self.position[1] -= other.position[1];
    }
}

impl ops::Mul<f32> for f32Vec2 {
    type Output = f32Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        f32Vec2 {
            position: [self.position[0] * rhs, self.position[1] * rhs],
        }
    }
}

impl ops::MulAssign<f32> for f32Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.position[0] *= rhs;
        self.position[1] *= rhs;
    }
}

impl ops::Div<f32> for f32Vec2 {
    type Output = f32Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        f32Vec2 {
            position: [self.position[0] / rhs, self.position[1] / rhs],
        }
    }
}

impl ops::DivAssign<f32> for f32Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.position[0] /= rhs;
        self.position[1] /= rhs;
    }
}

impl Display for f32Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.position[0], self.position[1])
    }
}
