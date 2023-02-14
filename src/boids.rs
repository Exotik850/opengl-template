use glium::implement_uniform_block;

pub(crate) const NUM_BOIDS: usize = 10000;

#[derive(Copy, Clone, Default)]
pub(crate) struct Boid {
    pos: [f32; 2],
    vel: [f32; 2],
    acc: [f32; 2],
}
implement_uniform_block!(Boid, pos, vel, acc);

#[derive(Copy, Clone)]
struct BoidBag([Boid; NUM_BOIDS]);
