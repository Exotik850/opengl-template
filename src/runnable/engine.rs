use drawable::shape::Shape;
use drawable::Drawable;
use glium::{glutin, Display, Program, Surface};
use landscape::Landscape;
use std::f32::consts::PI;
use std::time::SystemTime;
use winit::dpi::LogicalSize;
use winit::event::{Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

// The default shader that is stored in the engine
pub const BASE_VSHADER: &str = r#"
        #version 140
        in vec3 position;
        in vec3 normal;
        in vec3 world_position;
        in mat4 rotation_matrix;
        in vec4 color;
        uniform mat4 perspective;
        out vec3 v_normal;
        out vec4 v_col;

        void main() {
            v_col = color;
            v_normal = transpose(inverse(mat3(rotation_matrix))) * normal;
            vec4 pos = vec4(position, 1.0) * rotation_matrix * perspective;
            gl_Position = pos + vec4(world_position, 1.0);
        }
    "#;

pub const BASE_FSHADER: &str = r#"
        #version 140
        in vec3 v_normal;
        in vec4 v_col;
        uniform vec3 u_light;
        out vec4 color;
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            // vec4 dark_color = vec4(v_col.xyz * 0.6, v_col.w);
            vec4 dark_color = v_col;
            vec4 regular_color = v_col;
            color = mix(dark_color, regular_color, brightness);
        }
    "#;

pub struct Engine {
    pub objects: Vec<Landscape>,
    pub programs: Vec<Program>,
    pub display: Display,
}

// Trait for structs that hold a vector of objects that implement HasPos
// as well as a vector of programs (shaders) to draw the objects
impl Updatable for Engine {
    type RefType = Landscape;
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
        let start = SystemTime::now();
        let display = Self::default_display(event_loop);
        let obj = Landscape::default(&display);
        let programs =
            vec![Program::from_source(&display, BASE_VSHADER, BASE_FSHADER, None).unwrap()];
        println!(
            "Init time: {:?}",
            SystemTime::now().duration_since(start).unwrap()
        );
        Self {
            objects: vec![obj],
            programs,
            display,
        }
    }
}

// Trait that makes methods for references to all necessary objects
pub trait Updatable {
    type RefType: Drawable;
    type Type;

    // Make a default display given an event loop
    fn default_display(ev: &EventLoop<()>) -> Display {
        let wb = glutin::window::WindowBuilder::new()
            .with_title("OpenGL Template")
            .with_inner_size(LogicalSize {
                width: 800.0,
                height: 800.0,
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
    // Set up the engine
    fn init(event_loop: &EventLoop<()>) -> Self::Type;
}

// Trait for holding the main functions of the engine
pub trait Runnable<T>
where
    T: Drawable,
{
    fn window_handle(&mut self, window_event: &WindowEvent, control_flow: &mut ControlFlow);
    fn handle_events(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow);
    fn handle_keys(&mut self, _input: &KeyboardInput) {}
    fn update(&mut self) {}
    fn draw(&mut self);
}

impl<T, U> Runnable<T> for U
where
    U: Updatable<RefType = T>,
    T: Drawable,
{
    // Handle window closes and send keyboard inputs to key handler
    fn window_handle(&mut self, window_event: &WindowEvent, control_flow: &mut ControlFlow) {
        match window_event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => self.handle_keys(input),
            _ => (),
        }
    }

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
        objects.iter_mut().for_each(|obj| {
            obj.rotate_z(0.005);
            obj.update()
        });
        println!("Update time: {:?}", start.elapsed().unwrap());
    }

    fn draw(&mut self) {
        let start = SystemTime::now();

        // Grab the target frame from the display
        let mut target = self.ref_display().draw();
        // Clear the background
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 20.0);

        // Use the program at the index of the id of each object
        let programs = self.ref_programs();
        let objects = self.ref_objects();
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = PI / 2.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        for s in objects.iter() {
            // Draw the object onto the frame with the given shader
            s.draw(&mut target, &programs[0], &params, perspective);
        }

        // Finish with the frame
        target.finish().unwrap();
        println!("Frame time: {:?}", start.elapsed().unwrap());
    }
}
