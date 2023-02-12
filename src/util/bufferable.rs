use std::ops::{Index, IndexMut};
use glium::{Display, Vertex, VertexBuffer};

pub struct BufferObject<T: Vertex>
{
    data: Vec<T>,
    buffer: VertexBuffer<T>
}

impl<T: Vertex> Index<usize> for BufferObject<T>
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.data[index] }
}

impl<T: Vertex> IndexMut<usize> for BufferObject<T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.data[index] }
}


pub trait Bufferable {
    type Type: Vertex;
    fn new_vbo(display: &Display, vertices: &[Self::Type]) -> VertexBuffer<Self::Type> {
        VertexBuffer::new(display, &vertices).unwrap()
    }
}

impl<T: Vertex> Bufferable for T {
    type Type = T;
}
