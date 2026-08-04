use crate::Application;

pub struct Engine<A: Application> {
    app: A,
}

impl<A: Application> Engine<A> {
    pub fn new(app: A) -> Self {
        Self { app }
    }
}
