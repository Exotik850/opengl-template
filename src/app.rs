use engine::Runnable;
use glium::glutin::event::Event;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::{glutin, Display};
use shape::Drawable;
use std::any::Any;
use vertex::Manipulate;

pub struct App {
    pub event_loop: Option<EventLoop<()>>,
    pub display: Display,
}

impl App {
    pub fn new(event_loop: EventLoop<()>, display: Display) -> Self {
        App {
            event_loop: Some(event_loop),
            display,
        }
    }

    pub fn default_app() -> Self {
        let event_loop = EventLoop::new();
        let display = Self::default_display(&event_loop);
        Self::new(event_loop, display)
    }

    fn default_display(ev: &EventLoop<()>) -> Display {
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
        let display = Display::new(wb, cb, &ev).unwrap();
        display
    }

    fn grab_event_loop(&mut self) -> EventLoop<()> {
        match self.event_loop.take() {
            None => {
                panic!("Event loop already in use!")
            }
            Some(x) => x,
        }
    }

    fn handle_events<T, U>(&self, ev: &Event<()>, control_flow: &mut ControlFlow, engine: &mut T)
    where
        T: Runnable<U> + Any,
        U: Drawable<U> + Manipulate<U>,
    {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    engine.handle_keys(input);
                }
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => (),
            },
            Event::MainEventsCleared => {
                engine.update();
                self.display.gl_window().window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                engine.draw(&self.display);
            }
            _ => (),
        }
    }

    pub fn run<T, U>(mut self, mut engine: T)
    where
        T: Runnable<U> + Any,
        U: Drawable<U> + Manipulate<U>,
    {
        let event_loop = self.grab_event_loop();
        event_loop.run(move |ev, _, control_flow| {
            self.handle_events(&ev, control_flow, &mut engine);
        });
    }
}
