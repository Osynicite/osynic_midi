# Osynic MIDI to Keyboard Mapper - CLI Usage Guide

## Overview

`osynic-midi` is a command-line tool that maps MIDI input from connected MIDI devices to keyboard inputs. This allows you to use MIDI keyboards, controllers, and other MIDI devices to control applications that use keyboard input.

## Build Instructions

### Development Build
```bash
cargo build
```
Output: `target/debug/osynic-midi.exe`

### Release Build (Optimized)
```bash
cargo build --release
```
Output: `target/release/osynic-midi.exe`

## CLI Commands

### 1. List Available MIDI Devices
List all MIDI input devices currently connected to your system.

```bash
osynic-midi list-devices
```

**Example Output:**
```
Available MIDI input devices:
  0: X8III
  1: MIDIIN2 (X8III)
  2: loopMIDI Port
  3: loopMIDI Port 1
```

### 2. List Available Configuration Files
Display all available MIDI mapping configuration files in the `configs` directory.

```bash
osynic-midi list-configs
```

**Example Output:**
```
Available configuration files:
  0: midi_config.json
  1: midi_config_10ka.json
  2: midi_config_10kab.json
  ...
```

### 3. Start MIDI Mapping (Interactive)
Start the MIDI mapping with interactive prompts to select device, configuration, and mapping mode.

```bash
osynic-midi start
```

This will guide you through:
1. Selecting a configuration file
2. Selecting a MIDI input device
3. Choosing mapping mode (Notes or Octaves)

### 4. Start with Specific Configuration
Use a specific configuration file directly:

```bash
osynic-midi start --config configs/midi_config_10ka.json
```

Or shorthand:
```bash
osynic-midi start -c configs/midi_config_10ka.json
```

### 5. Start with Specific Mapping Mode
Choose the mapping mode directly:

```bash
osynic-midi start --mode notes
# or
osynic-midi start --mode octaves
```

Or shorthand:
```bash
osynic-midi start -m notes
```

### 6. Combined: Config + Mode
Specify both configuration and mode for non-interactive startup:

```bash
osynic-midi start -c configs/midi_config.json -m notes
```

## Mapping Modes

### Notes Mode
Maps individual MIDI notes to specific keyboard keys. This is based on the `note_mappings` section in the configuration file.

**Use case:** When you want specific keys for each note number.

### Octaves Mode
Maps MIDI notes based on octave and pitch name. This groups notes by octave and pitch (C, C#/Db, D, etc.).

**Use case:** When you have octave-based keyboard layouts.

## Configuration File Format

Configuration files are JSON-based and located in the `configs` directory.

### Example Structure:
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
    "0": "Key_0",
    "1": "Key_1",
    ...
    "127": "Key_127"
  }
}
```

### Fields Explained:

- **mapping_mode**: Default mode for the configuration ("notes" or "octaves"). Can be overridden by CLI argument.
- **velocity_threshold**: Minimum MIDI velocity (0-127) to trigger a key press. Keys below this threshold are ignored.
- **octaves**: Note mapping dictionary structured by octave number, then pitch name
- **note_mappings**: Direct note number (0-127) to key mapping

## Supported Key Names

The following key names are supported in configuration files:

### Letters:
A-Z (single uppercase letter)

### Symbols:
- `,` - Comma
- `.` - Period
- `/` - Forward slash
- `;` - Semicolon
- `'` - Apostrophe
- `[` - Left bracket
- `]` - Right bracket
- `\` - Backslash
- `-` - Hyphen
- `=` - Equals

### Special Keys:
- `Space` - Spacebar
- `Left` - Left arrow key
- `Right` - Right arrow key
- `RAlt` - Right Alt key

## Workflow Examples

### Example 1: Quick Start with Default Config and Notes Mode
```bash
osynic-midi start -c configs/midi_config.json -m notes
```

### Example 2: Interactive Setup
```bash
osynic-midi start
```
Follow the prompts to select configuration and mode.

### Example 3: Check Available Devices Before Starting
```bash
osynic-midi list-devices
osynic-midi list-configs
osynic-midi start
```

## Keyboard Shortcuts During Operation

- **Enter**: Exit the MIDI mapping application

## Error Handling

### Common Issues:

1. **No MIDI devices found**
   - Install MIDI device drivers
   - Check if devices are properly connected
   - Try using virtual MIDI loopback (e.g., loopMIDI on Windows)

2. **Configuration file not found**
   - Ensure config files are in the `configs` directory
   - Check the file path is correct
   - Use `list-configs` to see available files

3. **Invalid configuration file**
   - Verify JSON syntax is correct
   - Ensure all required fields are present
   - Check `velocity_threshold` is 0-127

4. **Keys not responding**
   - Check velocity_threshold setting (may be filtering out notes)
   - Verify mapping mode matches configuration structure
   - Ensure target application window has focus

## Creating Custom Configurations

1. Copy an existing configuration as a template:
```bash
copy configs/midi_config.json configs/my_config.json
```

2. Edit the file with a JSON editor:
- Add your MIDI note to key mappings
- Adjust velocity threshold if needed
- Set default mapping mode

3. Test with the new configuration:
```bash
osynic-midi start -c configs/my_config.json -m notes
```

## Performance Notes

- **Debug build** (`cargo build`): Slower startup, suitable for development
- **Release build** (`cargo build --release`): Optimized, ~1.1 MB executable
- MIDI latency is minimized using tokio async runtime

## Project Structure

```
osynic_midi/
├── src/
│   ├── main.rs          # CLI application (this is where the logic is)
│   ├── lib.rs           # Library exports
│   └── ...
├── examples/
│   └── ajm.rs          # Original example (now integrated into main.rs)
├── configs/
│   ├── midi_config.json
│   └── ...             # Various MIDI configuration files
└── Cargo.toml          # Project manifest
```

## Dependencies

- **serde/serde_json**: Configuration file parsing
- **tokio**: Async runtime for event handling
- **midir**: MIDI input handling
- **enigo**: Keyboard input simulation
- **clap**: Command-line argument parsing

## Building from Source

### Prerequisites:
- Rust 1.85.0 or later
- Windows 10/11 (uses Windows API for keyboard input)

### Steps:
```bash
git clone <repository>
cd osynic_midi
cargo build --release
```

The executable will be at `target/release/osynic-midi.exe`

## Tips and Best Practices

1. **Test your configuration first**
   - Use `list-devices` to ensure MIDI device is detected
   - Start with a simple configuration
   - Check that keys are being sent to the correct application

2. **Velocity handling**
   - Higher velocity_threshold filters light key presses
   - Set to 1-10 for piano-like controllers
   - Set to 0 to accept all velocities

3. **Multiple devices**
   - Run multiple instances of the tool with different MIDI devices
   - Each instance listens to one MIDI device only

4. **Keyboard focus**
   - Ensure the target application window is in focus
   - The mapper will send keystrokes to the active window

## Support and Troubleshooting

For detailed project information, see [README.md](README.md)

## License

MIT License - See LICENSE file for details
