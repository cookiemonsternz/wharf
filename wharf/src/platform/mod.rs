use crate::*;

#[cfg(feature = "winit")]
pub mod native;

pub fn run<A: Application>(engine: Engine<A>) {
    #[cfg(feature = "winit")]
    native::run(engine);
}
