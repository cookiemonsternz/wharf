use crate::*;

pub trait Service: Object {
    fn update(&mut self, delta: f64) {}
    fn fixed_update(&mut self) {}
    fn pre_render(&mut self) {}
    fn render(&mut self) {}
    fn post_render(&mut self) {}
    fn event(&mut self, event: &Event) -> bool {
        false
    }
}

impl PartialEq for dyn Service {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

pub struct ServiceScheduler {
    services: Vec<Box<dyn Service>>,
}

impl ServiceScheduler {
    pub fn register(&mut self, service: Box<dyn Service>) {
        self.services.push(service);
    }

    pub fn deregister(&mut self, service: Box<dyn Service>) {
        let index = self
            .services
            .iter()
            .position(|x| x.as_ref() == service.as_ref())
            .expect("could not find service in registered services");
        self.services.remove(index);
    }

    pub fn event(&mut self, event: &Event) -> bool {
        for service in self.services.iter_mut().rev() {
            let handled = service.event(event);
            if handled {
                return true;
            }
        }
        return false;
    }
}
