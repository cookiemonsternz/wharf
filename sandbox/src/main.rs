use wharf::*;

struct Sandbox;
impl Application for Sandbox {
    fn new() -> Self
    where
        Self: Sized,
    {
        Sandbox {}
    }

    fn run(&mut self) {
        let e = Event::WindowResize {
            width: 100,
            height: 100,
        };
        warn!("{}", e);
        loop {}
    }
}

entrypoint!(Sandbox);
