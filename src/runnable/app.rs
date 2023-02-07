use super::engine::Runnable;
use drawable::object::HasPos;
use glium::glutin::event_loop::EventLoop;
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;

// Holds the event loop that will run the engine
pub struct App {
    pub event_loop: RefCell<EventLoop<()>>,
}

impl App {
    pub fn new(event_loop: EventLoop<()>) -> Self {
        App {
            event_loop: RefCell::new(event_loop),
        }
    }

    pub fn default_app() -> Self {
        let event_loop = EventLoop::new();
        Self::new(event_loop)
    }

    pub fn event_loop_ref(&mut self) -> impl Deref<Target = EventLoop<()>> + '_ {
        self.event_loop.borrow()
    }

    pub fn grab_event_loop(self) -> EventLoop<()> {
        self.event_loop.into_inner()
    }

    // Runs any type that implements Updatable that holds a type of HasPos object
    pub fn run<T, U>(self, mut engine: T)
    where
        T: Runnable<U> + Any,
        U: HasPos,
    {
        let event_loop = self.grab_event_loop();
        event_loop.run(move |ev, _, control_flow| {
            engine.handle_events(&ev, control_flow);
        });
    }
}
