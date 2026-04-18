pub mod core;
pub mod create_config;
pub mod discovery;
pub mod error;
#[cfg(feature = "cli")]
pub mod interactive;
pub mod mapper;

// Re-export commonly used types
pub use core::{Config, KeyEvent, KeyboardMapper, MappingMode};
pub use create_config::{create_config_interactive, select_local_config};
pub use discovery::{display_configs, display_devices, list_configs, list_midi_devices};
#[cfg(feature = "cli")]
pub use interactive::{select_config, select_device, select_mode};
pub use mapper::start_mapping;
