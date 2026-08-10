use crate::*;

#[cfg(feature = "winit")]
pub mod native;

pub fn run(engine: Engine) {
    #[cfg(feature = "winit")]
    native::run(engine);
}
