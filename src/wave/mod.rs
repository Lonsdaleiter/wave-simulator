use crate::app::Application;
use crate::behavior::Behavior;
use crate::wave::bundles::basemetal::BaseMetalBundle;
use crate::wave::bundles::matrix::MatrixBundle;
use crate::wave::bundles::ui::UiBundle;
use crate::wave::bundles::water::{WaterBundle, Wave};
use crate::wave::bundles::window::WindowBundle;
use crate::wave::constants::FPS;
use crate::wave::keyboard::Keyboard;
use std::intrinsics::transmute;
use std::time::{Duration, Instant};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub mod behavior;
pub mod bundles;
pub mod camera;
pub mod constants;
pub mod keyboard;
pub mod widget;

pub struct WaveApp {
    pub keyboard: Keyboard,
    pub window_bundle: Option<WindowBundle>,
    pub base_metal_bundle: Option<BaseMetalBundle>,
    pub matrix_bundle: Option<MatrixBundle>,
    pub ui_bundle: Option<UiBundle>,
    pub water: Option<WaterBundle>,
    pub waves: [Option<Wave>; 8],
    pub time: u64,
    pub mouse_pos: (f64, f64),
}

impl Application for WaveApp {
    fn new() -> Self {
        WaveApp {
            keyboard: Keyboard { keys: [false; 300] },
            window_bundle: None,
            base_metal_bundle: None,
            matrix_bundle: None,
            ui_bundle: None,
            water: None,
            waves: [None; 8],
            time: 0,
            mouse_pos: (0.0, 0.0),
        }
    }

    fn execute(mut self, event_loop: EventLoop<()>) {
        self.window_bundle = Some(WindowBundle::new(&event_loop));

        let mut current_behavior: Box<dyn Behavior<Self>> =
            Box::new(behavior::loader::BaseLoaderBehavior);
        current_behavior.init(&mut self);

        let duration = Duration::from_millis((1000.0 / FPS) as u64);
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
                                current_behavior.init(&mut self);
                            }
                        }
                        now = Instant::now();
                        self.time += 1;
                    }
                    _ => {}
                },
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(size) => {
                        current_behavior.on_resize(&mut self, (size.width, size.height));
                    }
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    } => {
                        self.keyboard
                            .set_key(input.virtual_keycode.unwrap(), !unsafe {
                                transmute(input.state)
                            });
                    }
                    WindowEvent::MouseInput {
                        device_id: _,
                        state: _,
                        button: _,
                        modifiers: _,
                    } => {
                        // TODO add calls here
                    }
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        modifiers: _,
                    } => {
                        self.mouse_pos = (position.x, position.y);
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
