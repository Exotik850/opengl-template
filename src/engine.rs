use glium::{glutin, Display, Program, Surface};
use instance_group::InstanceGroup;
use object::HasPos;
use shape::{HasShape, Shape};
use std::ops::Index;
use std::time::SystemTime;
use winit::dpi::LogicalSize;
use winit::event::{Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

// The default shader that is stored in the engine
pub const BASE_VSHADER: &str = r#"
        #version 140
        in vec2 position;
        in vec2 world_position;
        in mat4 rotation_matrix;
        out vec2 my_attr;

        void main() {
            my_attr = position;
            vec4 pos = vec4(position , 0.0, 1.0) * rotation_matrix;
            gl_Position = pos + vec4(world_position, 0.0, 1.0);
        }
    "#;

pub const BASE_FSHADER: &str = r#"
        #version 140
        in vec2 my_attr;
        out vec4 color;
        void main() {
            color = vec4(my_attr, 0.0, 1.0);
        }
    "#;

pub struct Engine {
    pub objects: Vec<InstanceGroup<Shape>>,
    pub programs: Vec<Program>,
    pub display: Display,
}

// Trait for structs that hold a vector of objects that implement HasPos
// as well as a vector of programs (shaders) to draw the objects
impl Updatable for Engine {
    type RefType = InstanceGroup<Shape>;
    type Type = Engine;

    fn mut_objects(&mut self) -> &mut Vec<Self::RefType> {
        &mut self.objects
    }
    fn ref_objects(&self) -> &Vec<Self::RefType> {
        &self.objects
    }
    fn ref_programs(&self) -> &Vec<Program> {
        &self.programs
    }
    fn ref_display(&self) -> &Display {
        &self.display
    }

    // Set up an engine on a given event loop with predefined objects
    fn init(event_loop: &EventLoop<()>) -> Self::Type {
        let display = Self::default_display(event_loop);
        let obj = InstanceGroup::new(Shape::triangle(&display), 100000, &display);
        let programs =
            vec![Program::from_source(&display, BASE_VSHADER, BASE_FSHADER, None).unwrap()];
        Self::new(vec![obj], programs, display)
    }
    fn new(objects: Vec<Self::RefType>, programs: Vec<Program>, display: Display) -> Self::Type {
        Engine {
            objects,
            programs,
            display,
        }
    }
}

pub trait Updatable {
    type RefType: HasPos;
    type Type;

    // Make a default display given an event loop
    fn default_display(ev: &EventLoop<()>) -> Display {
        let wb = glutin::window::WindowBuilder::new()
            .with_title("OpenGL Template")
            .with_inner_size(LogicalSize {
                width: 600.0,
                height: 600.0,
            });
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = Display::new(wb, cb, &ev).unwrap();
        display
    }

    fn mut_objects(&mut self) -> &mut Vec<Self::RefType>;
    fn ref_objects(&self) -> &Vec<Self::RefType>;
    fn ref_programs(&self) -> &Vec<Program>;
    fn ref_display(&self) -> &Display;
    fn init(event_loop: &EventLoop<()>) -> Self::Type;
    fn new(objects: Vec<Self::RefType>, programs: Vec<Program>, display: Display) -> Self::Type;
}

pub trait Runnable<T>
where
    T: HasPos,
{
    // Handle window closes and send keyboard inputs to key handler
    fn window_handle(&mut self, window_event: &WindowEvent, control_flow: &mut ControlFlow) {
        match window_event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::KeyboardInput { input, .. } => {
                self.handle_keys(input);
            }
            _ => (),
        }
    }

    fn handle_events(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow);
    fn handle_keys(&mut self, _input: &KeyboardInput) {}
    fn update(&mut self) {}
    fn draw(&mut self);
}

impl<T, U> Runnable<T> for U
where
    U: Updatable<RefType = T>,
    T: HasPos,
{
    // Handle events to draw and update when window is done updating and drawing
    fn handle_events(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow) {
        match ev {
            Event::WindowEvent { event, .. } => self.window_handle(event, control_flow),
            Event::MainEventsCleared => {
                self.update();
                self.ref_display().gl_window().window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                self.draw();
            }
            _ => {}
        }
    }

    // Update the objects in the vector
    fn update(&mut self) {
        let start = SystemTime::now();
        let objects = self.mut_objects();
        objects.iter_mut().for_each(|obj| {obj.rotate(0.01); obj.update()});
        println!(
            "Update time: {:?}",
            SystemTime::now().duration_since(start).unwrap()
        );
    }

    fn draw(&mut self) {
        let start = SystemTime::now();

        // Grab the target frame from the display
        let mut target = self.ref_display().draw();
        // Clear the background
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Use the program at the index of the id of each object
        let programs = self.ref_programs();
        let objects = self.ref_objects();
        for s in objects.iter() {
            // Draw the object onto the frame with the given shader
            s.draw(&mut target, programs);
        }

        // Finish with the frame
        target.finish().unwrap();
        println!(
            "Frame time: {:?}",
            SystemTime::now().duration_since(start).unwrap()
        );
    }
}
