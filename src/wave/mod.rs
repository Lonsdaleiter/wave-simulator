use crate::app::Application;
use crate::behavior::Behavior;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::WindowBuilder;

mod behavior;

const FPS: f32 = 60.0;

pub struct WaveApp {
    //
}

impl Application for WaveApp {
    fn new() -> Self {
        WaveApp {}
    }

    fn execute(mut self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1280, 720))
            .with_title("Wave Simulator")
            .with_titlebar_transparent(true)
            .build(&event_loop)
            .unwrap();

        let mut behavior: Box<dyn Behavior<WaveApp>> =
            Box::new(behavior::load::ResourceLoadBehavior);

        let mut before = Instant::now();
        let duration = Duration::from_millis((1000.0 / FPS) as u64);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(before + duration);
            match event {
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached {
                        start: _,
                        requested_resume,
                    } => {
                        behavior.update(&mut self);
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
                        behavior.on_resize(&mut self);
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
