use glium::index::PrimitiveType;
use glium::{glutin, Display, Program, Surface};
use shape::Drawable;
use vertex::{Manipulate, Vertex};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

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

pub struct Engine<U>
where
    U: Drawable<U> + Manipulate<U>,
{
    pub shapes: Vec<U>,
    pub programs: Vec<Program>,
    pub display: Box<Display>,
}

pub trait Runnable<U>
where
    U: Drawable<U> + Manipulate<U>,
{
    fn default_display(ev: &EventLoop<()>) -> Display {
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = Display::new(wb, cb, &ev).unwrap();
        display
    }

    fn window_handle(&mut self, window_event: &WindowEvent, control_flow: &mut ControlFlow) {
        match window_event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
                return;
            }
            WindowEvent::KeyboardInput { input, .. } => {
                self.handle_keys(input.virtual_keycode.unwrap());
            }
            _ => return,
        }
    }

    fn handle_events(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow) {
        match ev {
            Event::WindowEvent { event, .. } => self.window_handle(event, control_flow),
            Event::MainEventsCleared => {
                self.update();
                self.display_ref().gl_window().window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                self.draw();
            }
            _ => (),
        }
    }

    fn add_shapes(&mut self, mut shapes: Vec<U>) {
        for shape in shapes.drain(..) {
            self.add_shape(shape);
        }
    }

    fn new(display: Display, shapes: Vec<U>, programs: Vec<Program>) -> Self;
    fn init(event_loop: &EventLoop<()>) -> Self;
    fn add_shape(&mut self, shape: U);
    fn display_ref(&mut self) -> &Display;
    fn handle_keys(&mut self, input: VirtualKeyCode) {}
    fn update(&mut self) {}
    fn draw(&mut self) {
        let mut target = self.display_ref().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();
    }
}

impl<U> Runnable<U> for Engine<U>
where
    U: Drawable<U> + Manipulate<U>,
{
    fn new(display: Display, shapes: Vec<U>, programs: Vec<Program>) -> Engine<U> {
        Engine {
            display: Box::new(display),
            shapes,
            programs,
        }
    }

    fn init(event_loop: &EventLoop<()>) -> Self {
        let vertex1 = Vertex {
            position: [-0.5, -0.5],
        };
        let vertex2 = Vertex {
            position: [0.0, 0.5],
        };
        let vertex3 = Vertex {
            position: [0.5, -0.5],
        };
        let vertices = vec![vertex1, vertex2, vertex3, vertex2 * 2.0f32];
        let index_type = PrimitiveType::LineStrip;

        let display = Self::default_display(event_loop);

        let shape = U::new(&display, vertices, index_type, 0);
        let mut shape2 = shape.clone(&display);
        shape2.rotate(45.0);

        let shapes: Vec<U> = vec![shape, shape2];

        let program = Program::from_source(&display, BASE_VSHADER, BASE_FSHADER, None).unwrap();
        let programs = vec![program];
        Self::new(display, shapes, programs)
    }

    fn add_shape(&mut self, shape: U) {
        self.shapes.push(shape);
    }

    fn display_ref(&mut self) -> &Display {
        &*self.display
    }

    fn update(&mut self) {
        for s in self.shapes.iter_mut() {
            s.rotate(0.1);
        }
    }

    fn draw(&mut self) {
        let mut target = self.display_ref().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for s in self.shapes.iter_mut() {
            let program = self.programs.get(s.get_id() as usize).unwrap();
            s.draw(&mut target, program);
        }

        target.finish().unwrap();
    }
}
