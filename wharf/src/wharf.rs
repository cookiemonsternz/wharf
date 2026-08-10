pub mod engine;
pub mod entry_point;
pub mod event;
pub mod object;
pub mod service;

// Re-exports
pub use engine::Engine;
pub use event::Event;
pub use event::KeyCode;
pub use event::MouseCode;
pub use object::Object;

pub use log::{debug, error, info, trace, warn};
pub use uuid::Uuid;

pub use entry_point::run;

pub fn init() {
    pretty_env_logger::init();
    info!("Wharf initialized");
}

pub fn shutdown() {
    info!("Wharf shutdown");
}
