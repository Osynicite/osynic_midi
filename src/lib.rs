pub mod error;
pub mod core;
pub mod discovery;
pub mod interactive;
pub mod mapper;
pub mod create_config;

// Re-export commonly used types
pub use core::{ Config, MappingMode, KeyboardMapper, KeyEvent };
pub use discovery::{ list_configs, list_midi_devices, display_configs, display_devices };
pub use interactive::{ select_config, select_device, select_mode };
pub use mapper::start_mapping;
pub use create_config::{ create_config_interactive, select_local_config };
