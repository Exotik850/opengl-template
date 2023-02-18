use std::iter::Zip;

pub(crate) mod attribute;
pub(crate) mod bufferable;
mod compute_container;
pub(crate) mod vertex;

pub trait Manipulate {
    fn rotate_axis(&mut self, axis: usize, ang: f32);
}
