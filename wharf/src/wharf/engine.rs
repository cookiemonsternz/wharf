use crate::*;
use log::info;

#[derive(Debug)]
pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn on_event(&mut self, event: Event) {
        info!("Engine: {}", event);
    }
}
