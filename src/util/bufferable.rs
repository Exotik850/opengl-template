use glium::vertex::PerInstance;
use glium::{Display, Vertex, VertexBuffer};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use util::Manipulate;

pub struct BufferObject<T: Vertex + Manipulate> {
    data: Box<Vec<T>>,
    buffer: Box<VertexBuffer<T>>,
}

// unsafe impl<T: Vertex> Send for BufferObject<T> {}

impl<T: Vertex + Manipulate> Index<usize> for BufferObject<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Vertex + Manipulate> IndexMut<usize> for BufferObject<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Vertex + Manipulate> BufferObject<T> {
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn update_buffer(&self) {
        self.buffer.write(&self.data);
    }

    pub fn ref_data(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn mut_data(&mut self) -> &mut [T] {
        self.data.as_mut()
    }

    pub fn ref_vbo(&self) -> &VertexBuffer<T> {
        &self.buffer
    }

    pub fn per_instance(&self) -> PerInstance<'_> {
        self.buffer.per_instance().unwrap()
    }
}

pub trait Bufferable {
    type Type: Vertex + Manipulate;
    fn new_vbo(display: &Display, vertices: &[Self::Type]) -> BufferObject<Self::Type> {
        let data = Box::from(Vec::from(vertices));
        let buffer = Box::from(VertexBuffer::new(display, &vertices).unwrap());
        BufferObject { data, buffer }
    }
}

impl<T: Vertex + Manipulate> Bufferable for T {
    type Type = T;
}

impl<T: Vertex + Manipulate> Manipulate for BufferObject<T> {
    fn rotate_axis(&mut self, axis: usize, ang: f32) {
        self.data.iter_mut().for_each(|p| p.rotate_axis(axis, ang));
    }
}
