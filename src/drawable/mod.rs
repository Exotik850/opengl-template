pub(crate) mod instance_group;
pub(crate) mod shape;
pub(crate) mod shape_group;

use glium::{DrawParameters, Frame, Program};

pub trait Drawable {
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        perspective: [[f32; 4]; 4],
    );

    fn update(&mut self) {}
    fn rotate_z(&mut self, angle: f32);
}
