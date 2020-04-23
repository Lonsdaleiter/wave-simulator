use crate::app::Application;

pub trait Actor<A: Application> {
    fn act(&self, app: &mut A);
}
