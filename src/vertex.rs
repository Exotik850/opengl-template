use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct F32vec3 {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}
glium::implement_vertex!(F32vec3, position, normal);

#[derive(Copy, Clone, Debug)]
pub struct Attr {
    pub world_position: [f32; 3],
    pub rotation_matrix: [[f32; 4]; 4],
}
glium::implement_vertex!(Attr, world_position, rotation_matrix);

impl Default for Attr {
    fn default() -> Self {
        let world_position = [0.0; 3];
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
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.world_position[0] += x;
        self.world_position[1] += y;
        self.world_position[2] += z;
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.world_position[0] = x;
        self.world_position[1] = y;
        self.world_position[2] = z;
    }

    pub fn x(&self) -> f32 {
        self.world_position[0]
    }

    pub fn y(&self) -> f32 {
        self.world_position[1]
    }

    pub fn z(&self) -> f32 {
        self.world_position[2]
    }

    pub fn rotate_z(&mut self, ang: f32) {
        let cos_theta = ang.cos();
        let sin_theta = ang.sin();
        for i in 0..4 {
            let x = self.rotation_matrix[i][0];
            let y = self.rotation_matrix[i][1];
            self.rotation_matrix[i][0] = cos_theta * x - sin_theta * y;
            self.rotation_matrix[i][1] = sin_theta * x + cos_theta * y;
        }
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        for i in 0..4 {
            let x = self.rotation_matrix[i][0];
            let z = self.rotation_matrix[i][2];
            self.rotation_matrix[i][0] = cos_theta * x + sin_theta * z;
            self.rotation_matrix[i][2] = -sin_theta * x + cos_theta * z;
        }
    }

    pub fn rand(&mut self) {
        self.world_position.iter_mut().for_each(|p| {
            *p = thread_rng().gen_range(-2.0..2.0);
        });
    }
}

#[allow(dead_code)]
impl F32vec3 {
    pub fn x(&self) -> f32 {
        self.position[0]
    }
    pub fn y(&self) -> f32 {
        self.position[1]
    }
    pub fn z(&self) -> f32 {
        self.position[2]
    }
    pub fn mag_sq(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
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

    pub fn rotateZ(&mut self, ang: f32) {
        let (x, y) = (self.x(), self.y());
        let (cs, sn) = (ang.cos(), ang.sin());
        self.position[0] = x * cs - y * sn;
        self.position[1] = x * sn + y * cs;
    }

    pub fn lerp(&self, other: &Self, amt: f32) -> F32vec3 {
        *other * amt + *self * (1.0 - amt)
    }

    pub fn dot(&self, other: &Self) -> F32vec3 {
        F32vec3::from([
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        ])
    }

    pub fn dot_prod(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() + other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> F32vec3 {
        F32vec3::from([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ])
    }
}

impl Default for F32vec3 {
    fn default() -> Self {
        Self::from([0.0; 3])
    }
}

impl Into<[f32; 3]> for F32vec3 {
    fn into(self) -> [f32; 3] {
        self.position
    }
}

impl From<&[f32; 3]> for F32vec3 {
    fn from(value: &[f32; 3]) -> Self {
        F32vec3 {
            position: *value,
            normal: [0.0, 0.0, -1.0],
        }
    }
}

impl From<[f32; 3]> for F32vec3 {
    fn from(value: [f32; 3]) -> Self {
        F32vec3 {
            position: value,
            normal: [0.0, 0.0, -1.0],
        }
    }
}

impl ops::Add for F32vec3 {
    type Output = F32vec3;
    fn add(self, other: Self) -> Self::Output {
        F32vec3::from([
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ])
    }
}

impl ops::AddAssign for F32vec3 {
    fn add_assign(&mut self, other: Self) {
        self.position[0] += other.position[0];
        self.position[1] += other.position[1];
        self.position[2] += other.position[2];
    }
}

impl ops::Sub for F32vec3 {
    type Output = F32vec3;
    fn sub(self, other: Self) -> Self::Output {
        F32vec3::from([
            self.position[0] - other.position[0],
            self.position[1] - other.position[1],
            self.position[2] - other.position[2],
        ])
    }
}

impl ops::SubAssign for F32vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.position[0] -= other.position[0];
        self.position[1] -= other.position[1];
        self.position[2] -= other.position[2];
    }
}

impl ops::Mul<f32> for F32vec3 {
    type Output = F32vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        F32vec3::from([
            self.position[0] * rhs,
            self.position[1] * rhs,
            self.position[2] * rhs,
        ])
    }
}

impl ops::MulAssign<f32> for F32vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.position[0] *= rhs;
        self.position[1] *= rhs;
        self.position[2] *= rhs;
    }
}

impl ops::Div<f32> for F32vec3 {
    type Output = F32vec3;
    fn div(self, rhs: f32) -> Self::Output {
        F32vec3::from([
            self.position[0] / rhs,
            self.position[1] / rhs,
            self.position[2] / rhs,
        ])
    }
}

impl ops::DivAssign<f32> for F32vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.position[0] /= rhs;
        self.position[1] /= rhs;
        self.position[2] /= rhs;
    }
}

impl Display for F32vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.position[0], self.position[1], self.position[2]
        )
    }
}
