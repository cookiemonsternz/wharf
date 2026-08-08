use crate::*;

pub trait Object {
    fn init(&mut self) {}
    fn deinit(&mut self) {}

    fn id(&self) -> Uuid;
}

impl PartialEq for dyn Object {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
