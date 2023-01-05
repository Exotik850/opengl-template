use glium::index::PrimitiveType;
use glium::{Display, Program, Surface};
use shape::Drawable;
use vertex::{Manipulate, Vertex};
use winit::event::KeyboardInput;

pub const BASE_VSHADER: &str = r#"
        #version 140

        in vec2 position;
        out vec2 my_attr;

        uniform mat4 matrix;

        void main() {
            my_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

pub const BASE_FSHADER: &str = r#"
        #version 140

        in vec2 my_attr;
        out vec4 color;

        void main() {
            color = vec4(my_attr + vec2(0.5, 0.5), 0.0, 1.0);
        }
    "#;

pub struct Engine<U: Drawable<U> + Manipulate<U>> {
    pub shapes: Vec<U>,
    pub programs: Vec<Program>,
}

pub trait Runnable<U>
where
    U: Drawable<U> + Manipulate<U>,
{
    fn new(shapes: Vec<U>, programs: Vec<Program>) -> Self;
    fn handle_keys(&mut self, input: &KeyboardInput) {}
    fn update(&mut self) {}
    fn draw(&mut self, display: &Display) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();
    }
}

impl<U> Runnable<U> for Engine<U>
where
    U: Drawable<U> + Manipulate<U>,
{
    fn new(shapes: Vec<U>, programs: Vec<Program>) -> Engine<U> {
        Engine { shapes, programs }
    }

    fn update(&mut self) {
        for s in self.shapes.iter_mut() {
            s.rotate(0.1);
        }
    }

    fn draw(&mut self, display: &Display) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for s in self.shapes.iter_mut() {
            let program = self.programs.get(s.get_id() as usize).unwrap();
            s.draw(&mut target, program);
        }

        target.finish().unwrap();
    }
}
