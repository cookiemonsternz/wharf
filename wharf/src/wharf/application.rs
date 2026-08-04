pub trait Application {
    // To be defined in client
    fn new() -> Self
    where
        Self: Sized;

    fn run(&mut self);
}
