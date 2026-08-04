use super::application::Application;
use super::engine::Engine;

pub fn run_application<T: Application>() {
    crate::init();

    let app = T::new();
    let engine = Engine::new(app);
    crate::platform::run(engine);

    crate::shutdown();
}

#[macro_export]
/// First argument is a struct type implementing Application
macro_rules! entrypoint {
    ($app:ty) => {
        fn main() {
            $crate::run_application::<$app>();
        }
    };
}
