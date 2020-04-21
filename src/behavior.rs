use crate::app::Application;

pub trait Behavior<T: Application> {
    fn update(&self, state: &mut T);
    fn draw(&self, state: &mut T);
    fn on_resize(&self, state: &mut T);
    fn on_destroy(&self, state: &mut T);
}
