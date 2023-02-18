use rand::{thread_rng, Rng};
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
        let (cs, sn) = (ang.cos(), ang.sin());
        let mut rot_matrix = self.rotation_matrix;
        for i in 0..4 {
            let (a, b) = match (axis, i) {
                (0, 1) => (cs, -sn),
                (0, 2) => (sn, cs),
                (1, 0) => (cs, sn),
                (1, 2) => (-sn, cs),
                (2, 0) => (cs, -sn),
                (2, 1) => (sn, cs),
                _ => (1.0, 0.0),
            };
            rot_matrix[i][0] = a * self.rotation_matrix[i][0] + b * self.rotation_matrix[i][2];
            rot_matrix[i][2] = -b * self.rotation_matrix[i][0] + a * self.rotation_matrix[i][2];
        }
        self.rotation_matrix = rot_matrix;
    }
}
