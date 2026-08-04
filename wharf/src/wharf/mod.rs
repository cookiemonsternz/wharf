pub mod application;
pub mod engine;
pub mod entry_point;
pub mod event;

// Re-exports
pub use application::Application;
pub use entry_point::run_application;
pub use event::Event;
pub use event::KeyCode;
pub use event::MouseCode;
pub use log::{debug, error, info, trace, warn};

use pretty_env_logger;

pub fn init() {
    pretty_env_logger::init();
    info!("Wharf initialized");
}

pub fn shutdown() {
    info!("Wharf shutdown");
}
