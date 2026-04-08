# Osynic MIDI Library API Documentation

## Overview

The `osynic_midi` crate is a modular MIDI to keyboard mapping library that can be used both as:
1. **A CLI tool** - `osynic-midi` command-line application
2. **A library** - Embedded in other Rust applications for programmatic MIDI mapping

## Architecture

The library is organized into the following modules:

### 1. `core` - Core MIDI Mapping Logic

Core data structures and the keyboard mapping engine.

#### Types

```rust
pub enum MappingMode {
    Octaves,  // Maps by octave and pitch
    Notes,    // Maps individual notes
}

pub struct Config {
    pub mapping_mode: Option<MappingMode>,
    pub octaves: HashMap<String, HashMap<String, String>>,
    pub velocity_threshold: u8,
    pub note_mappings: HashMap<u8, String>,
}

pub enum KeyEvent {
    NoteOn(u8, u8),  // (note, velocity)
    NoteOff(u8),     // note
}

pub struct KeyboardMapper {
    pub config: Config,
    pub enigo: Arc<Mutex<Enigo>>,
    pub mode: MappingMode,
}
```

#### Functions

```rust
impl Config {
    /// Load configuration from JSON file
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>>

    /// Convert MIDI note number to octave and pitch
    pub fn note_to_pitch(note: u8) -> (u8, String)

    /// Get keyboard key for a MIDI note
    pub fn get_key_for_note(&self, note: u8, mode: &MappingMode) -> Option<Key>
}

impl KeyboardMapper {
    /// Create a new mapper with configuration
    pub fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self

    /// Handle a MIDI key event
    pub fn handle_event(&self, event: KeyEvent) -> Result<(), Box<dyn Error>>
}
```

### 2. `discovery` - Configuration and Device Discovery

Functions for finding available MIDI configurations and devices.

#### Functions

```rust
/// List all configuration files in the 'configs' directory
pub fn list_configs() -> Result<Vec<String>, Box<dyn Error>>

/// List all available MIDI input devices
pub fn list_midi_devices() -> Result<Vec<String>, Box<dyn Error>>

/// Display formatted list of available configurations
pub fn display_configs() -> Result<(), Box<dyn Error>>

/// Display formatted list of available MIDI devices
pub fn display_devices() -> Result<(), Box<dyn Error>>
```

### 3. `interactive` - Interactive Menu Selection

Interactive prompts for user selection (using `inquire` library).

#### Functions

```rust
/// Prompt user to select a configuration file
pub fn select_config() -> Result<String, Box<dyn Error>>

/// Prompt user to select a MIDI input device (returns device index)
pub fn select_device() -> Result<usize, Box<dyn Error>>

/// Prompt user to select mapping mode
pub fn select_mode() -> Result<MappingMode, Box<dyn Error>>
```

### 4. `mapper` - MIDI Mapping Control

Main MIDI mapping function for starting the mapping process.

#### Functions

```rust
/// Start MIDI to keyboard mapping
pub async fn start_mapping(
    config_path: String,
    device_idx: usize,
    mode: MappingMode,
) -> Result<(), Box<dyn Error>>
```

## Usage Examples

### Example 1: Basic Configuration Loading

```rust
use osynic_midi::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("configs/midi_config.json")?;
    println!("Velocity threshold: {}", config.velocity_threshold);
    println!("Note mappings: {}", config.note_mappings.len());
    Ok(())
}
```

### Example 2: Discovering Resources

```rust
use osynic_midi::{list_configs, list_midi_devices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // List available configurations
    let configs = list_configs()?;
    println!("Found {} configuration files", configs.len());

    // List available MIDI devices
    let devices = list_midi_devices()?;
    println!("Found {} MIDI devices", devices.len());

    Ok(())
}
```

### Example 3: Interactive Selection

```rust
use osynic_midi::interactive::{select_config, select_device, select_mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let user select configuration
    let config_path = select_config()?;
    
    // Let user select device
    let device_idx = select_device()?;
    
    // Let user select mode
    let mode = select_mode()?;

    println!("Selected: {} on device {}", config_path, device_idx);
    Ok(())
}
```

### Example 4: Programmatic MIDI Mapping

```rust
use osynic_midi::{
    Config, MappingMode, KeyboardMapper, KeyEvent,
};
use enigo::{Enigo, Settings};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load("configs/midi_config.json")?;
    
    // Create mapper
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mapper = KeyboardMapper::new(
        config,
        enigo,
        MappingMode::Notes,
    );

    // Handle a MIDI note
    let event = KeyEvent::NoteOn(60, 100);  // Middle C, velocity 100
    mapper.handle_event(event)?;

    Ok(())
}
```

### Example 5: Complete MIDI Mapping

```rust
use osynic_midi::start_mapping;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_mapping(
        "configs/midi_config.json".to_string(),
        0,  // First MIDI device
        osynic_midi::MappingMode::Notes,
    ).await?;

    Ok(())
}
```

## Feature Flags

The library supports different features:

```toml
[features]
default = ["lib"]
full = ["lib", "cli"]
cli = []           # CLI features (clap, inquire)
lib = []           # Library core features
```

To use just the library without CLI:
```toml
osynic_midi = { version = "0.1", default-features = false, features = ["lib"] }
```

## Integration with External Applications

### Creating a Custom Application

Here's how to build a custom MIDI mapper using the library:

```rust
use osynic_midi::{
    Config, MappingMode, KeyboardMapper, KeyEvent,
    list_configs, list_midi_devices,
};
use std::sync::{Arc, Mutex};
use enigo::{Enigo, Settings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Discover
    let configs = list_configs()?;
    let devices = list_midi_devices()?;

    // 2. Initialize
    let config = Config::load(&format!("configs/{}", configs[0]))?;
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mapper = KeyboardMapper::new(config, enigo, MappingMode::Notes);

    // 3. Use mapper
    let event = KeyEvent::NoteOn(64, 100);
    mapper.handle_event(event)?;

    Ok(())
}
```

### Building a Custom CLI

Using the library to create a different command interface:

```rust
use osynic_midi::{start_mapping, list_configs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = "configs/midi_config.json".to_string();
    let device = 0;
    let mode = osynic_midi::MappingMode::Notes;

    start_mapping(config, device, mode).await
}
```

## Configuration File Format

Configuration files are JSON files in the `configs/` directory:

```json
{
    "mapping_mode": "notes",
    "velocity_threshold": 0,
    "octaves": {
        "3": {
            "C": "A",
            "C#/Db": "S",
            "D": "D",
            ...
        }
    },
    "note_mappings": {
        "60": "C",
        "61": "C#",
        ...
    }
}
```

## Error Handling

All functions return `Result<T, Box<dyn Error>>` for flexibility:

```rust
match Config::load("config.json") {
    Ok(config) => println!("Loaded successfully"),
    Err(e) => eprintln!("Failed to load: {}", e),
}
```

## Thread Safety

Key components are thread-safe:
- `KeyboardMapper` uses `Arc<Mutex<Enigo>>` for concurrent access
- All MIDI event handling is async-safe with tokio channels

## Performance

- **Binary size**: ~1.3 MB (release build)
- **Startup time**: < 100ms
- **Memory overhead**: ~5-10 MB idle
- **Latency**: < 10ms for MIDI event handling

## API Stability

The library API is designed to be stable:
- Core types (`Config`, `MappingMode`, `KeyEvent`) are stable
- Functions follow Rust conventions
- Error handling is consistent

## Examples

See the `examples/` directory for more complete examples:
- `lib_usage.rs` - Basic library usage demonstration
- `ajm.rs` - Original example code (now in library)

## Building

As a library:
```bash
cargo build --lib
```

With CLI:
```bash
cargo build --features cli
cargo build --release --features cli
```

## Testing

Run the example:
```bash
cargo run --example lib_usage
```

Run tests:
```bash
cargo test
```

## Future Enhancements

Potential library improvements:
- [ ] Device hot-plug detection
- [ ] Configuration validation API
- [ ] Events API for external subscribers
- [ ] Multiple device support
- [ ] Configuration builder pattern
- [ ] Async MIDI device enumeration

## License

MIT License - See LICENSE file for details

---

**For CLI usage**, see [CLI_USAGE.md](../CLI_USAGE.md)  
**For interactive upgrades**, see [INTERACTIVE_MENU_UPDATE.md](../INTERACTIVE_MENU_UPDATE.md)
