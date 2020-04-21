use crate::app::Application;
use crate::wave::WaveApp;
use winit::event_loop::EventLoop;

mod app;
mod behavior;
mod wave;

fn main() {
    let event_loop = EventLoop::new();
    let wave_app = WaveApp::new(&event_loop);
    wave_app.execute(event_loop);
}
