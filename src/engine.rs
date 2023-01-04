use glium::glutin::event_loop::{EventLoop};
use glium::{Display, glutin, Program, Surface};
use glium::glutin::event::Event;
use glium::index::PrimitiveType;
use shape::{Drawable, Shape};
use vertex::{Manipulate, Vertex};

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


pub struct Engine<U: Drawable<U> + Manipulate<U>>{
    pub display: Display,
    pub shapes: Vec<U>,
    pub programs: Vec<Program>,
}

pub trait Runnable<U>
where U: Drawable<U> + Manipulate<U>
{
    fn default_display(ev: &EventLoop<()>) -> Display {
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &ev).unwrap();
    display
    }

    fn new(display: Display, shapes: Vec<U>, programs: Vec<Program>) -> Self;
    fn init(ev: &EventLoop<()>) -> Self;
    fn event_handle(&mut self, ev: Event<()>){}
    fn update(&mut self) {}
    fn draw(&mut self) {}
}

impl<U> Runnable<U> for Engine<U>
where U: Drawable<U> + Manipulate<U>
{
    fn new(display: Display, shapes: Vec<U>, programs: Vec<Program>) -> Engine<U> {
        Engine{
            display,
            shapes,
            programs
        }
    }

    fn init(ev: &EventLoop<()>) -> Engine<U> {
        let display = Self::default_display(ev);

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [0.0, 0.5] };
        let vertex3 = Vertex { position: [0.5, -0.5] };
        let vertices = vec![vertex1, vertex2, vertex3, vertex2 * 2.0f32];
        let index_type = PrimitiveType::TrianglesList;

        let shape = U::new(&display, vertices, index_type, 0);
        let mut shape2 = shape.clone(&display);
        shape2.rotate(45.0);

        let shapes: Vec<U> = vec![shape, shape2];

        let program =
            Program::from_source(&display, BASE_VSHADER, BASE_FSHADER, None).unwrap();
        let programs = vec![program];

        Engine::new(display, shapes, programs)
    }

    fn update(&mut self) {
        for s in self.shapes.iter_mut(){
            s.rotate(0.01);
        }
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for s in self.shapes.iter_mut(){
            let program = self.programs.get(s.get_id() as usize).unwrap();
            s.draw(&mut target, program);
        }

        target.finish().unwrap();
    }

}