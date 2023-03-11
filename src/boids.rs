use drawable::shape::Shape;
use drawable::shape_group::ShapeGroup;
use drawable::{DrawUniforms, Drawable};
use glium::{implement_uniform_block, Display, DrawParameters, Frame, Program};
use util::attribute::Attr;
use util::bufferable::Bufferable;
use util::vertex::F32vec3;
use util::Manipulate;

pub(crate) const NUM_BOIDS: usize = 10000;

pub struct Boids {
    shapegroup: ShapeGroup<Shape>,
    velocities: Vec<F32vec3>,
    accelerations: Vec<F32vec3>,
}

impl Boids {
    pub fn default(display: &Display, num: usize) -> Self {
        let mut shapegroup = ShapeGroup::default();
        let circle = Shape::circle(display, 0.03, 10);
        let mut attributes = vec![];
        let mut velocities = vec![];
        let mut accelerations = vec![];

        for i in 0..num {
            attributes.push(Attr::random());
            velocities.push(F32vec3::random() * 0.001);
            accelerations.push(F32vec3::default());
        }

        let attr = Attr::new_vbo(display, &attributes);
        shapegroup.push((circle, attr));

        Self {
            shapegroup,
            velocities,
            accelerations,
        }
    }
}

impl Drawable for Boids {
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        uniforms: DrawUniforms,
    ) {
        self.shapegroup.draw(target, program, params, uniforms);
    }

    fn update(&mut self) {
        for (ind, boid) in self.shapegroup.iter_mut_transforms(0).enumerate() {
            let force = F32vec3::from([
                -boid.world_position[0] * 0.0001,
                -boid.world_position[1] * 0.0001,
                0.0,
            ]);
            self.velocities[ind] += force;
            self.velocities[ind].limit(5.0);
            *boid += F32vec3::from(self.velocities[ind]);
        }
        println!("{:?}", self.velocities[0]);
        self.shapegroup.update_buffers();
    }
}

impl Manipulate for Boids {
    fn rotate_axis(&mut self, axis: usize, ang: f32) {
        todo!()
    }
}
