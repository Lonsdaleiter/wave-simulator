pub trait Application {
    fn new() -> Self;
    fn execute(self);
}
