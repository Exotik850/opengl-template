use glium::{Display, Vertex, VertexBuffer};
use rand::{thread_rng, Rng};


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

    pub fn rotate_x(&mut self, angle: f32) {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        for i in 0..4 {
            let y = self.rotation_matrix[i][1];
            let z = self.rotation_matrix[i][2];
            self.rotation_matrix[i][1] = cos_theta * y - sin_theta * z;
            self.rotation_matrix[i][2] = sin_theta * y + cos_theta * z;
        }
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

