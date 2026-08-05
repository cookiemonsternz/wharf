use log::info;

use crate::{Application, Event};

#[derive(Debug)]
pub struct Engine<A: Application> {
    app: A,
}

impl<A: Application> Engine<A> {
    pub fn new(app: A) -> Self {
        Self { app }
    }

    pub fn on_event(&mut self, event: Event) {
        info!("Engine: {}", event);
    }
}
