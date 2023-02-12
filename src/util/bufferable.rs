use glium::vertex::PerInstance;
use glium::{Display, Vertex, VertexBuffer};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

pub struct BufferObject<T: Vertex> {
    data: Box<Vec<T>>,
    buffer: Box<VertexBuffer<T>>,
}

impl<T: Vertex> Index<usize> for BufferObject<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Vertex> IndexMut<usize> for BufferObject<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Vertex> BufferObject<T> {
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
    type Type: Vertex;
    fn new_vbo(display: &Display, vertices: &[Self::Type]) -> BufferObject<Self::Type> {
        let data = Box::from(Vec::from(vertices));
        let buffer = Box::from(VertexBuffer::new(display, &vertices).unwrap());
        BufferObject { data, buffer }
    }
}

impl<T: Vertex> Bufferable for T {
    type Type = T;
}
