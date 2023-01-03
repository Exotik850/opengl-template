use na::{vector, Vector2, U2};
use nalgebra as na;
use rand::random;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub pos: Vector2<f64>,
    pub vel: Vector2<f64>,
    pub acc: Vector2<f64>,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Particle {
        Particle {
            pos: vector![x, y],
            vel: vector![0.0, 0.0],
            acc: vector![0.0, 0.0],
        }
    }

    pub fn rand() -> Particle {
        Particle {
            pos: vector![random(), random()].normalize(),
            vel: vector![random(), random()].normalize(),
            acc: vector![random(), random()].normalize(),
        }
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        self.vel = self.vel.cap_magnitude(5.0);
        self.pos += self.vel;
    }

    pub fn add_force(&mut self, force: Vector2<f64>) {
        self.acc += force;
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {} : {}", self.pos, self.vel, self.acc)
    }
}
