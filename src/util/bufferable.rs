use glium::{Display, Vertex, VertexBuffer};

pub trait Bufferable {
    type Type: Vertex;
    fn new_vbo(display: &Display, vertices: &[Self::Type]) -> VertexBuffer<Self::Type> {
        VertexBuffer::new(display, &vertices).unwrap()
    }
}

impl<T: Vertex> Bufferable for T {
    type Type = T;
}
