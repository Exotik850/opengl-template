use drawable::shape::Shape;
use rand::{thread_rng, Rng};
use std::ops;
use util::vertex::F32vec3;
use util::Manipulate;

#[derive(Copy, Clone, Debug)]
pub struct Attr {
    pub world_position: [f32; 3],
    pub rotation_matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
}
glium::implement_vertex!(Attr, world_position, rotation_matrix, color);

impl Default for Attr {
    fn default() -> Self {
        let world_position = [0.0; 3];
        let rotation_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let color = [1.0, 0.0, 0.0, 1.0];
        Attr {
            world_position,
            rotation_matrix,
            color,
        }
    }
}

impl From<&[f32; 3]> for Attr {
    fn from(value: &[f32; 3]) -> Self {
        let rotation_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let color = [1.0, 0.0, 0.0, 1.0];
        Attr {
            world_position: *value,
            rotation_matrix,
            color,
        }
    }
}

impl From<[f32; 3]> for Attr {
    fn from(value: [f32; 3]) -> Self {
        let rotation_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let color = [1.0, 0.0, 0.0, 1.0];
        Attr {
            world_position: value,
            rotation_matrix,
            color,
        }
    }
}

#[allow(dead_code)]
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

    pub fn randomize(&mut self) {
        self.world_position
            .iter_mut()
            .for_each(|p| *p = thread_rng().gen_range(-2.0..2.0));
        self.color
            .iter_mut()
            .for_each(|p| *p = thread_rng().gen_range(0.0..1.0));
    }

    pub fn random() -> Self {
        let mut a = Attr::default();
        a.randomize();
        a
    }
}

impl Manipulate for Attr {
    fn rotate_axis(&mut self, axis: usize, ang: f32) {
        let cos = ang.cos();
        let sin = ang.sin();
        let mut tmp: [f32; 4] = [0.0; 4];

        for i in 0..4 {
            tmp[i] = self.rotation_matrix[axis][i];
        }

        for i in 0..4 {
            let a = tmp[i];
            let b = self.rotation_matrix[axis ^ 1][i];
            let c = self.rotation_matrix[axis ^ 2][i];

            self.rotation_matrix[axis][i] = a * cos - b * sin;
            self.rotation_matrix[axis ^ 1][i] = a * sin + b * cos;
            self.rotation_matrix[axis ^ 2][i] = c;
        }
    }
}

impl ops::Add<F32vec3> for Attr {
    type Output = Attr;
    fn add(self, other: F32vec3) -> Self::Output {
        Attr::from([
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ])
    }
}

impl ops::AddAssign<F32vec3> for Attr {
    fn add_assign(&mut self, other: F32vec3) {
        self.world_position[0] += other.position[0];
        self.world_position[1] += other.position[1];
        self.world_position[2] += other.position[2];
    }
}

impl ops::Sub<F32vec3> for Attr {
    type Output = Attr;
    fn sub(self, other: F32vec3) -> Self::Output {
        Attr::from([
            self.world_position[0] - other.position[0],
            self.world_position[1] - other.position[1],
            self.world_position[2] - other.position[2],
        ])
    }
}

impl ops::SubAssign<F32vec3> for Attr {
    fn sub_assign(&mut self, other: F32vec3) {
        self.world_position[0] -= other.position[0];
        self.world_position[1] -= other.position[1];
        self.world_position[2] -= other.position[2];
    }
}
