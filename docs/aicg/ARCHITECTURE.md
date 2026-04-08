# Osynic MIDI - Project Architecture & Module Structure

## Overview

The project is structured as a Rust library with a CLI wrapper, allowing both:
1. **Direct library usage** - Use the library in your own Rust projects
2. **Standalone CLI** - Use `osynic-midi` as a command-line tool

## Directory Structure

```
osynic_midi/
├── src/
│   ├── main.rs              # CLI entry point (thin wrapper)
│   ├── lib.rs               # Library root (public API)
│   ├── core.rs              # Core MIDI mapping logic ✨ NEW
│   ├── discovery.rs         # Config & device discovery ✨ NEW
│   ├── interactive.rs       # Interactive menu selection ✨ NEW
│   ├── mapper.rs            # MIDI mapping control ✨ NEW
│   ├── config.rs            # Configuration (existing)
│   ├── error.rs             # Error types (existing)
│   ├── keyboard.rs          # Keyboard mapping (existing)
│   └── midi.rs              # MIDI handling (existing)
├── examples/
│   ├── ajm.rs               # Original example (refactored)
│   └── lib_usage.rs         # Library usage example ✨ NEW
├── configs/                 # MIDI configuration files
├── LIBRARY_API.md           # Library API documentation ✨ NEW
├── ARCHITECTURE.md          # This file ✨ NEW
├── CLI_USAGE.md             # CLI usage guide
├── INTERACTIVE_MENU_UPDATE.md
├── DEVELOPMENT_COMPLETE.md
└── Cargo.toml               # Project manifest
```

## Module Organization

### `lib.rs` - Library Root
**Purpose**: Expose public API for external users

```rust
pub mod core;
pub mod discovery;
pub mod interactive;
pub mod mapper;

// Re-export commonly used types
pub use core::{Config, MappingMode, KeyboardMapper, KeyEvent};
pub use discovery::{list_configs, list_midi_devices, ...};
pub use interactive::{select_config, select_device, select_mode};
pub use mapper::start_mapping;
```

**What's exported:**
- All core types for MIDI mapping
- Discovery functions for finding configs/devices
- Interactive functions for menu selection
- Main async function for starting mapping

### `core.rs` - Core Logic (NEW)
**Purpose**: Core MIDI mapping functionality independent of CLI

**Types:**
- `MappingMode` enum - Notes or Octaves
- `Config` struct - Configuration from JSON
- `KeyEvent` enum - MIDI note events
- `KeyboardMapper` struct - Maps MIDI to keyboard

**Key features:**
- Configuration loading and validation
- Note-to-pitch conversion
- Key mapping resolution
- MIDI event handling

**Dependencies:** enigo (keyboard), serde (JSON)

### `discovery.rs` - Discovery (NEW)
**Purpose**: Find and list available options

**Functions:**
- `list_configs()` - Find JSON config files
- `list_midi_devices()` - Enumerate MIDI ports
- `display_configs()` - Pretty-print configs
- `display_devices()` - Pretty-print devices

**Use case:** Both CLI and library users need this

### `interactive.rs` - Interactive Selection (NEW)
**Purpose**: User-friendly menu selection

**Functions:**
- `select_config()` - Pick a config file
- `select_device()` - Pick a MIDI device
- `select_mode()` - Pick mapping mode

**Technology:** Uses `inquire` library for arrow-key navigation

**Use case:** CLI primarily, but can be used in library apps too

### `mapper.rs` - Mapping Control (NEW)
**Purpose**: Start and run MIDI mapping

**Functions:**
- `start_mapping()` - Async function to run mapping

**Features:**
- MIDI device connection
- Event channel setup
- Async event handler
- Graceful shutdown

**Async model:** Spawns tokio task for event processing

### `main.rs` - CLI Entry Point (REFACTORED)
**Purpose**: Command-line interface for users

**Pattern:** Thin wrapper around library functions

**Flow:**
```
Usage: osynic-midi <COMMAND>
│
├─ Commands::ListDevices  ─→ discovery::display_devices()
├─ Commands::ListConfigs  ─→ discovery::display_configs()
├─ Commands::Start        ─→ start_cli_mapping()
│  │
│  ├─ interactive::select_config()
│  ├─ interactive::select_device()
│  ├─ interactive::select_mode()
│  └─ mapper::start_mapping()
│
└─ Default               ─→ start_cli_mapping()
```

## Data Flow

### Initialization
```
Load Config (core::Config::load)
    ↓
Create Mapper (core::KeyboardMapper::new)
    ↓
Access Enigo (for keyboard control)
```

### MIDI Event
```
MIDI Device
    ↓
MidiInput callback
    ↓
Create KeyEvent (core::KeyEvent)
    ↓
Send to channel (tokio mpsc)
    ↓
Async handler task
    ↓
mapper.handle_event()
    ↓
Key press/release via Enigo
```

### Configuration
```
discovery::list_configs()
    ↓
Read configs/ directory
    ↓
Filter .json files
    ↓
Sort and return names
    ↓
interactive::select_config()
    ↓
Show in inquire menu
    ↓
core::Config::load()
    ↓
Parse JSON and validate
```

## Separation of Concerns

### Library (`lib.rs` exports)
- **core**: MIDI mapping logic
- **discovery**: Finding resources
- **interactive**: Menu selection
- **mapper**: Control mapping

### CLI (`main.rs`)
- Parse command-line arguments (clap)
- Chain library functions
- Display results to user
- Handle errors gracefully

## API Boundaries

### Public (library users)
```rust
// In lib.rs
pub use core::{Config, MappingMode, KeyboardMapper, KeyEvent};
pub use discovery::{list_configs, list_midi_devices, ...};
pub use interactive::{select_config, select_device, select_mode};
pub use mapper::start_mapping;
```

### Internal (CLI only)
```rust
// In main.rs
fn start_cli_mapping(...) -> Result<()>
```

## Dependency Graph

```
main.rs
  ├─ clap (CLI args)
  ├─ lib::discovery
  ├─ lib::interactive
  └─ lib::mapper

lib::mapper
  ├─ lib::core
  └─ midir (MIDI)

lib::core
  ├─ enigo (keyboard)
  ├─ serde (JSON)
  └─ tokio::sync::mpsc

lib::discovery
  ├─ midir (MIDI ports)
  └─ std::fs (file listing)

lib::interactive
  ├─ inquire (menu UI)
  └─ lib::discovery

Cargo.toml:
  ├─ serde + serde_json
  ├─ tokio
  ├─ midir
  ├─ enigo
  ├─ clap
  └─ inquire
```

## Usage Patterns

### Pattern 1: CLI User
```bash
osynic-midi start
# → select_config() → select_device() → select_mode() → start_mapping()
```

### Pattern 2: Direct Library Integration
```rust
let config = Config::load("my_config.json")?;
let mapper = KeyboardMapper::new(config, enigo, mode);
```

### Pattern 3: Full Programmatic Control
```rust
start_mapping("config.json".to_string(), 0, MappingMode::Notes).await?;
```

## Testing Architecture

### Unit Tests
- Would test `core::Config` parsing
- Would test note-to-pitch conversion
- Would test key mapping logic

### Integration Tests
- Test discovery functions
- Test interactive selections (mock)
- Test mapper initialization

### CLI Tests
- Test argument parsing
- Test command routing
- Test error messages

## Future Enhancements

### Library Extensions
1. **Event API**
   ```rust
   pub trait MidiEventListener {
       fn on_note_on(&self, note: u8, velocity: u8);
       fn on_note_off(&self, note: u8);
   }
   ```

2. **Builder Pattern**
   ```rust
   MappingBuilder::new()
       .config("path.json")
       .device(0)
       .mode(MappingMode::Notes)
       .build()
   ```

3. **Device Enumeration**
   ```rust
   pub async fn watch_devices() -> Result<Receiver<DeviceEvent>>
   ```

### CLI Enhancements
1. Configuration validation before starting
2. Status display (device name, mode, note count)
3. Help for each command

## Build Targets

```bash
# Library only
cargo build --lib

# With CLI
cargo build --features cli

# Full featured
cargo build --release --features cli

# Run example
cargo run --example lib_usage
```

## Module Relationship Summary

```
                    ┌─────────────────┐
                    │   main.rs (CLI) │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │   lib.rs        │
                    │  (Public API)   │
                    └────────┬────────┘
                             │
        ┌────────────┬───────┼───────┬────────────┐
        │            │       │       │            │
    ┌───▼───┐  ┌────▼──┐ ┌──▼────┐ ┌▼────────┐ ┌▼─────────┐
    │ core  │  │mapper │ │ inter │ │  disco  │ │ (legacy) │
    │ MIDI  │  │ async │ │active │ │ very    │ │  config  │
    │ logic │  │ run   │ │ menus │ │ resources│ │  error   │
    └───────┘  └──┬────┘ └──┬────┘ └─────────┘ └──────────┘
                   │        │
              ┌────▼────┬───▼──────┐
              │          │          │
           ┌──▼──┐  ┌───▼──┐  ┌───▼──┐
           │midi │  │ key  │  │serde │
           │dir  │  │board │  │json  │
           └─────┘  └──────┘  └──────┘
```

## Key Design Decisions

1. **Modular separation**: Core logic separate from CLI
2. **Async I/O**: Tokio for MIDI event handling
3. **Type safety**: No string-based errors
4. **Reusability**: Library functions usable independently
5. **Thin CLI**: CLI is a thin wrapper, not a separate app

## Maintenance Notes

- **Adding new config fields**: Modify `core::Config`
- **Adding CLI commands**: Add variant to `Commands` enum in `main.rs`
- **Adding key mappings**: Extend `get_key_for_note()` in `core::Config`
- **Changing MIDI logic**: Update `mapper::start_mapping()`
- **New interactive menu**: Add function in `interactive.rs`

---

This architecture makes the codebase:
- ✅ **Modular** - Easy to test each component
- ✅ **Reusable** - Library can be used elsewhere
- ✅ **Maintainable** - Clear separation of concerns
- ✅ **Extensible** - Easy to add new features
- ✅ **Professional** - Follows Rust best practices
