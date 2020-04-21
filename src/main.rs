use crate::app::Application;
use crate::wave::WaveApp;

mod app;
mod behavior;
mod wave;

fn main() {
    let mut wave_app = WaveApp::new();
    wave_app.execute();
}
