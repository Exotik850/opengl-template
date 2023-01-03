use crate::shape::Shape;
use crate::vertex::Manipulate;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::{glutin, Program, Surface};

pub struct App {
    display: glium::Display,
    shapes: Vec<Shape>,
    programs: Vec<Program>,
    ev: Option<EventLoop<()>>,
}

impl App {
    pub fn new() -> Self {
        let ev = Some(EventLoop::new());
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, ev.as_ref().unwrap()).unwrap();

        let tri = Shape::triangle(&display, 0);
        let shapes = vec![tri];

        let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 my_attr;
        
        uniform mat4 matrix;
        
        void main() {
            my_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

        let fragment_shader_src = r#"
        #version 140

        in vec2 my_attr;
        out vec4 color;
        
        void main() {
            color = vec4(my_attr + vec2(0.5, 0.5), 0.0, 1.0);
        }
    "#;

        let program =
            Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let programs = vec![program];

        App {
            display,
            shapes,
            programs,
            ev,
        }
    }

    pub fn update(&mut self) {
        for s in self.shapes.iter_mut() {
            s.rotate(0.1);
            // println!("{:?}", s.va);
        }
    }

    pub fn draw(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for s in self.shapes.iter_mut() {
            let program = &self.programs.get(s.id as usize).unwrap();
            s.draw(&mut target, &program);
        }

        target.finish().unwrap();
    }

    pub fn run(mut self) {
        let handler = match self.ev.take() {
            Some(ev) => ev,
            None => panic!("Event handler not found (Have you already tried to run this?)"),
        };

        handler.run(move |ev, _, control_flow: &mut ControlFlow| {
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }
            let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(5000);
            *control_flow = ControlFlow::WaitUntil(next_frame_time);

            self.update();
            self.draw();
        });
    }
}
