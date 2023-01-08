use std::cell::RefCell;
use std::fs::soft_link;
use std::slice::IterMut;
use glium::{glutin, Display, Program, Surface};
use glium::index::PrimitiveType;
use shape::{Drawable, HasShape, Shape};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use object::{HasPos, Object};
use vertex::f32Vec2;

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

pub struct Engine

{
    pub objects: Vec<Object>,
    pub programs: RefCell<Vec<Program>>,
    pub display: Display,
}

impl Updatable for Engine
{
    type RefType = Object;
    type Type = Engine;

    fn ref_objects(&self) -> &Vec<Self::RefType> { &self.objects }
    fn mut_objects(&mut self) -> &mut Vec<Self::RefType> { &mut self.objects }

    fn get_programs(&mut self) -> Vec<Program> {
        self.programs.take()
    }

    fn ref_display(&self) -> &Display { &self.display }
    fn add_object(&mut self, shape: Self::RefType) { self.objects.push(shape) }
    fn init(event_loop: &EventLoop<()>) -> Self::Type
    {
        let vertex1 = f32Vec2{ position: [1.0, -1.0] };
        let vertex2 = f32Vec2{ position: [-1.0, -1.0]};
        let vertex3 = f32Vec2{ position: [0.0, 1.0]};
        let vertices = vec![vertex1, vertex2, vertex3];
        let display = Self::default_display(event_loop);
        let shape = Shape{
            vertices: vertices.clone(),
            vbo: Shape::new_vbo(&display, &vertices),
            index_type: PrimitiveType::TrianglesList,
            id: 0,
        };
        let obj = Object::from_shape(shape);
        let programs = vec![Program::from_source(&display, BASE_VSHADER, BASE_FSHADER, None).unwrap()];
        Self::new(vec![obj], programs, display)
    }
    fn new(objects: Vec<Object>, programs: Vec<Program>, display: Display) -> Self::Type
    { Engine { objects, programs: RefCell::new(programs), display } }
}

pub trait Updatable
{
    type RefType;
    type Type;
    fn default_display(ev: &EventLoop<()>) -> Display {
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = Display::new(wb, cb, &ev).unwrap();
        display
    }

    fn ref_objects(&self) -> &Vec<Self::RefType>;
    fn mut_objects(&mut self) -> &mut Vec<Self::RefType>;
    fn get_programs(&mut self) -> Vec<Program>;
    fn ref_display(&self) -> &Display;
    fn add_object(&mut self, shape: Self::RefType);
    fn add_objects(&mut self, mut shapes: Vec<Self::RefType>) {
        for shape in shapes.drain(..) { self.add_object(shape); }
    }
    fn init(event_loop: &EventLoop<()>) -> Self::Type;
    fn new(objects: Vec<Self::RefType>, programs: Vec<Program>, display: Display) -> Self::Type;
}

pub trait Runnable<T>
where
    T: HasPos
{
    fn window_handle(&mut self, window_event: &WindowEvent, control_flow: &mut ControlFlow) {
        match window_event {
            WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; }
            WindowEvent::KeyboardInput { input, .. } => { self.handle_keys(input.virtual_keycode.unwrap()); }
            _ => (),
        }
    }

    fn handle_events(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow);
    fn handle_keys(&mut self, _input: VirtualKeyCode) {}
    fn update(&mut self) {}
    fn draw(&mut self);
}

impl<T, U> Runnable<T> for U
where
    U: Updatable<RefType=T>,
    T: HasPos
{
    fn handle_events(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow) {}

    fn update(&mut self) {
        for s in self.mut_objects().iter_mut() {
            s.update();
        }
    }

    fn draw(&mut self) {
        let mut target = self.ref_display().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let programs = self.get_programs();
        for s in self.mut_objects().iter_mut() {
            let program = programs.get(s.get_id() as usize).unwrap();
            s.draw(&mut target, program);
        }
        target.finish().unwrap();
    }
}