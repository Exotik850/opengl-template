use std::any::Any;
use glium::glutin;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::event::Event;
use engine::{Runnable};

pub struct App{
    pub event_loop: EventLoop<()>,
}

impl App
{
    pub fn new(event_loop: EventLoop<()>) -> Self{
        App{
            event_loop,
        }
    }

    pub fn default_app() -> Self{
        let event_loop = EventLoop::new();
        Self::new(event_loop)
    }

    pub fn event_handle(ev: Event<()>, control_flow: &mut ControlFlow){
        match ev {
            Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(5000);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);
    }

    pub fn run<T: Runnable<T> + Any>(self, mut engine: T) {
        self.event_loop.run(move |ev, _, control_flow: &mut ControlFlow| {
            Self::event_handle(ev, control_flow);
            engine.update();
            engine.draw();
        });
    }
}