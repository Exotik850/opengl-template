use engine::Runnable;
use glium::glutin::event_loop::{EventLoop};
use shape::Drawable;
use std::any::Any;
use vertex::Manipulate;

pub struct App {
    pub event_loop: Box<EventLoop<()>>,
}

impl App {
    pub fn new(event_loop: EventLoop<()>) -> Self {
        App {
            event_loop: Box::new(event_loop),
        }
    }

    pub fn default_app() -> Self {
        let event_loop = EventLoop::new();
        Self::new(event_loop)
    }

    pub fn event_loop_ref(&mut self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub fn grab_event_loop(self) -> EventLoop<()> {
        *self.event_loop
    }

    pub fn run<T, U>(self, mut engine: T)
    where
        T: Runnable<U> + Any,
        U: Drawable<U> + Manipulate<U>,
    {
        let event_loop = self.grab_event_loop();
        event_loop.run(move |ev, _, control_flow| {
            engine.handle_events(&ev, control_flow);
        });
    }
}
