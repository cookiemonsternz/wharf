#[cfg(feature = "winit")]
pub mod native;

use crate::{Application, wharf::engine::Engine};

pub fn run<A: Application>(engine: Engine<A>) {
    #[cfg(feature = "winit")]
    native::run(engine);
}
