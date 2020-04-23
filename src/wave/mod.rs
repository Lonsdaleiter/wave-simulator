use crate::app::Application;
use crate::behavior::Behavior;
use std::time::{Duration, Instant};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub mod behavior;
pub mod bundles;
pub mod constants;

pub struct WaveApp;
impl Application for WaveApp {
    fn new() -> Self {
        WaveApp
    }

    fn execute(mut self, event_loop: EventLoop<()>) {
        let mut current_behavior: Box<dyn Behavior<Self>> =
            Box::new(behavior::loader::window::WindowBehavior);

        let duration = Duration::from_millis((1000.0 / constants::FPS) as u64);
        let mut now = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(now + duration);
            match event {
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached {
                        start: _,
                        requested_resume: _,
                    } => {
                        let nb = current_behavior.update(&mut self);
                        match nb {
                            None => {}
                            Some(t) => {
                                current_behavior.on_death(&mut self);
                                current_behavior = t;
                            }
                        }
                        now = Instant::now();
                    }
                    _ => {}
                },
                Event::WindowEvent { window_id: _, event } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(size) => {
                        current_behavior.on_resize(&mut self, (size.width, size.height));
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    current_behavior.draw(&mut self);
                }
                Event::LoopDestroyed => {
                    current_behavior.on_death(&mut self);
                }
                _ => {}
            }
        });
    }
}
