pub(crate) mod instance_group;
pub(crate) mod shape;
pub(crate) mod shape_group;

use boids::{Boid, NUM_BOIDS};
use glium::uniforms::UniformValue;
use glium::{DrawParameters, Frame, Program};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct DrawUniforms {
    perspective: [[f32; 4]; 4],
    u_light: [f32; 3],
}

impl Default for DrawUniforms {
    fn default() -> Self {
        let u_light = [-1.0, 0.4, 0.9f32];
        let perspective = {
            let (width, height) = (800.0, 800.0);
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = PI / 2.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };
        Self {
            u_light,
            perspective,
        }
    }
}

impl glium::uniforms::Uniforms for DrawUniforms {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("perspective", UniformValue::Mat4(self.perspective));
        f("u_light", UniformValue::Vec3(self.u_light));
    }
}

pub trait Drawable {
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        uniforms: DrawUniforms,
    );

    fn update(&mut self) {}
    fn rotate_z(&mut self, angle: f32);
}
