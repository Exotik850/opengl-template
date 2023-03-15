use glium::index::PrimitiveType;
use glium::{Display, VertexBuffer};
use std::f32::consts::PI;
use util::bufferable::{BufferObject, Bufferable};
use util::vertex::F32vec3;

const TWO_PI: f32 = PI * 2.0;

// Struct for handling the components of a primitive shape and drawing it to the screen
pub struct Shape {
    pub vertices: BufferObject<F32vec3>,
    pub index_type: PrimitiveType,
    pub id: usize,
}

impl Shape {
    // Default shape
    pub fn triangle(display: &Display) -> Shape {
        let vertex1 = F32vec3::from([1.0, -1.0, 0.0]) * 0.1;
        let vertex2 = F32vec3::from([-1.0, -1.0, 0.0]) * 0.1;
        let vertex3 = F32vec3::from([0.0, 1.0, 0.0]) * 0.1;
        let vertices = F32vec3::new_vbo(display, &vec![vertex1, vertex2, vertex3]);
        Shape {
            vertices,
            index_type: PrimitiveType::TrianglesList,
            id: 0,
        }
    }

    pub fn quad(display: &Display, scl: f32) -> Shape {
        let mut vertices = vec![
            F32vec3::from([-1.0, 1.0, 0.0]),
            F32vec3::from([1.0, 1.0, 0.0]),
            F32vec3::from([-1.0, -1.0, 0.0]),
            F32vec3::from([1.0, -1.0, 0.0]),
        ];
        vertices.iter_mut().for_each(|p| *p *= scl);
        let vertices = F32vec3::new_vbo(display, &vertices);
        Shape {
            vertices,
            index_type: PrimitiveType::TriangleStrip,
            id: 0,
        }
    }

    pub fn circle(display: &Display, radius: f32, num: u32) -> Shape {
        let mut vertices = vec![];

        let inc = TWO_PI / num as f32;
        let mut ang = 0.0;
        while ang <= TWO_PI + 0.0005 {
            let x = ang.cos() * radius;
            let y = ang.sin() * radius;
            vertices.push(F32vec3::from([x, y, 0.0]));
            ang += inc;
        }

        let vertices = F32vec3::new_vbo(display, &vertices);
        Shape {
            vertices,
            index_type: PrimitiveType::LineStrip,
            id: 0,
        }
    }

    pub fn from_vertices(
        vertices: &Vec<F32vec3>,
        index_type: PrimitiveType,
        display: &Display,
    ) -> Shape {
        Shape {
            vertices: F32vec3::new_vbo(display, &vertices),
            index_type,
            id: 0,
        }
    }
}

// Trait for making sure any other shape made has the same types and handles drawing
pub trait HasShape {
    type RefType;
    fn ref_vertices(&self) -> &[F32vec3];
    fn mut_vertices(&mut self) -> &mut [F32vec3];
    fn ref_vbo(&self) -> &VertexBuffer<F32vec3>;
    fn ref_index(&self) -> &PrimitiveType;
    fn get_id(&self) -> usize;
    fn update_vbo(&self) {
        self.ref_vbo().write(self.ref_vertices())
    }
}

impl HasShape for Shape {
    type RefType = Shape;
    fn ref_vertices(&self) -> &[F32vec3] {
        &self.vertices.ref_data()
    }
    fn mut_vertices(&mut self) -> &mut [F32vec3] {
        self.vertices.mut_data()
    }
    fn ref_vbo(&self) -> &VertexBuffer<F32vec3> {
        self.vertices.ref_vbo()
    }
    fn ref_index(&self) -> &PrimitiveType {
        &self.index_type
    }
    fn get_id(&self) -> usize {
        self.id
    }
}

unsafe impl Send for Shape {}
