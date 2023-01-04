use std::any::Any;
use glium::glutin;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::event::Event;
use engine;
use engine::{Runnable};
use shape::Drawable;
use vertex::Manipulate;

enum State{
    Running,
    Exit
}

pub struct App{
    pub event_loop: Box<EventLoop<()>>,
}

impl App
{
    pub fn new(event_loop: EventLoop<()>) -> Self{
        App{
            event_loop: Box::new(event_loop),
        }
    }

    pub fn default_app() -> Self{
        let event_loop = EventLoop::new();
        Self::new(event_loop)
    }

    fn check_closed(ev: &Event<()>) -> State{
        match ev {
            Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    State::Exit
                }
                _ => State::Running,
            },
            Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => State::Running,
                glutin::event::StartCause::Init => State::Running,
                _ => State::Running,
            },
            _ => State::Running,
        }
    }

    fn wait(time: u64) -> std::time::Instant{
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(time);
        next_frame_time
    }

    pub fn run<T, U>(self, engine: T)
    where T: Runnable<U> + Any,
          U: Drawable<U> + Manipulate<U>
    {
        let event_loop = *self.event_loop;
        let mut engine = engine;
        event_loop.run(move |ev, _, control_flow| {
            let state = Self::check_closed(&ev);
            match state{
                State::Running => (),
                State::Exit => {*control_flow = ControlFlow::Exit}
            }

            engine.update();
            engine.draw();
            engine.event_handle(ev);
            *control_flow = ControlFlow::WaitUntil(Self::wait(5000));

        });
    }
}