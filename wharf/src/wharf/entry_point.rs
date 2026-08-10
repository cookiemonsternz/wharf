use crate::*;

pub fn run() {
    crate::init();

    let engine = Engine::new();
    crate::platform::run(engine);

    crate::shutdown();
}

#[macro_export]
macro_rules! entrypoint {
    () => {
        fn main() {
            $crate::run();
        }
    };
}
