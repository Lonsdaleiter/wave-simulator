use crate::app::Application;
use crate::behavior::Behavior;
use crate::wave::bundles::window::WindowBundle;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::WindowBuilder;
use crate::wave::bundles::resource::ResourceBundle;

mod behavior;
mod bundles;

const FPS: f32 = 60.0;

pub struct WaveApp {
    pub window_bundle: WindowBundle,
    pub resource_bundle: Option<ResourceBundle>,
}

impl Application for WaveApp {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1280, 720))
            .with_title("Wave Simulator")
            .with_titlebar_transparent(true)
            .build(&event_loop)
            .unwrap();

        WaveApp {
            window_bundle: WindowBundle { window },
            resource_bundle: None,
        }
    }

    fn execute(mut self, event_loop: EventLoop<()>) {
        let mut behavior: Box<dyn Behavior<WaveApp>> =
            Box::new(behavior::load::ResourceLoadBehavior);
        behavior.init(&mut self);

        let mut before = Instant::now();
        let duration = Duration::from_millis((1000.0 / FPS) as u64);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(before + duration);
            match event {
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached {
                        start: _,
                        requested_resume: _,
                    } => {
                        let b = behavior.update(&mut self);
                        match b {
                            Some(b) => {
                                b.init(&mut self);
                                behavior = b
                            }
                            _ => {}
                        };

                        before = Instant::now();
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    behavior.draw(&mut self);
                }
                Event::LoopDestroyed => {
                    behavior.on_destroy(&mut self);
                }
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::Resized(size) => {
                        behavior.on_resize(&mut self, (size.width, size.height));
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}
