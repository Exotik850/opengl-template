use glium::index::{NoIndices, PrimitiveType};
use glium::{uniform, Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use util::bufferable::Bufferable;
use util::vertex::F32vec3;

// Struct for handling the components of a primitive shape and drawing it to the screen
pub struct Shape {
    pub vertices: Vec<F32vec3>,
    pub vbo: VertexBuffer<F32vec3>,
    pub index_type: PrimitiveType,
    pub id: usize,
}

impl Shape {
    // Default shape
    pub fn triangle(display: &Display) -> Shape {
        let vertex1 = F32vec3::from([1.0, -1.0, 0.0]) * 0.1;
        let vertex2 = F32vec3::from([-1.0, -1.0, 0.0]) * 0.1;
        let vertex3 = F32vec3::from([0.0, 1.0, 0.0]) * 0.1;
        let vertices = vec![vertex1, vertex2, vertex3];
        Shape {
            vertices: vertices.clone(),
            vbo: F32vec3::new_vbo(&display, &vertices),
            index_type: PrimitiveType::TrianglesList,
            id: 0,
        }
    }

    pub fn quad(display: &Display) -> Shape {
        let mut vertices = vec![
            F32vec3::from([-1.0, 1.0, 0.0]),
            F32vec3::from([1.0, 1.0, 0.0]),
            F32vec3::from([-1.0, -1.0, 0.0]),
            F32vec3::from([1.0, -1.0, 0.0]),
        ];
        for i in vertices.iter_mut() {
            *i *= 0.1;
        }
        Shape {
            vertices: vertices.clone(),
            vbo: F32vec3::new_vbo(&display, &vertices),
            index_type: PrimitiveType::TriangleStrip,
            id: 0,
        }
    }

    pub fn from_vertices(
        vertices: &Vec<F32vec3>,
        index_type: PrimitiveType,
        display: &Display,
    ) -> Shape {
        Shape {
            vertices: vertices.clone(),
            vbo: F32vec3::new_vbo(&display, &vertices),
            index_type,
            id: 0,
        }
    }
}

// Trait for making sure any other shape made has the same types and handles drawing
pub trait HasShape {
    type RefType;
    fn ref_vertices(&self) -> &Vec<F32vec3>;
    fn mut_vertices(&mut self) -> &mut Vec<F32vec3>;
    fn ref_vbo(&self) -> &VertexBuffer<F32vec3>;
    fn ref_index(&self) -> &PrimitiveType;
    fn get_id(&self) -> usize;

    fn update_vbo(&self) {
        self.ref_vbo().write(self.ref_vertices())
    }

    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        perspective: [[f32; 4]; 4],
    ) {
        self.update_vbo();
        let uniforms = uniform! {
            u_light: [-1.0, 0.4, 0.9f32],
            perspective: perspective
        };
        target
            .draw(
                self.ref_vbo(),
                &NoIndices(*self.ref_index()),
                &program,
                &uniforms,
                &params,
            )
            .unwrap();
    }
}

impl HasShape for Shape {
    type RefType = Shape;
    fn ref_vertices(&self) -> &Vec<F32vec3> {
        &self.vertices
    }
    fn mut_vertices(&mut self) -> &mut Vec<F32vec3> {
        &mut self.vertices
    }
    fn ref_vbo(&self) -> &VertexBuffer<F32vec3> {
        &self.vbo
    }
    fn ref_index(&self) -> &PrimitiveType {
        &self.index_type
    }
    fn get_id(&self) -> usize {
        self.id
    }
}
