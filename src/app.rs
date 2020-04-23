use winit::event_loop::EventLoop;

pub trait Application {
    fn new() -> Self;
    fn execute(self, event_loop: EventLoop<()>);
}
