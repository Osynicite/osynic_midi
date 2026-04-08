<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="OsynicMIDI"/>
</p>

<p align="center">
  <h1 align="center">🎵 OsynicMIDI</h1>
  <p align="center"><strong>An Elegant MIDI to Keyboard Mapper Library in Rust</strong></p>
  <p align="center">Map MIDI devices (keyboards, controllers, etc.) to keyboard keys and control any application with ease</p>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue?style=flat-square"/></a>
  <a href="https://crates.io/crates/osynic_midi" target="_blank"><img src="https://img.shields.io/crates/v/osynic_midi?style=flat-square"/></a>
  <a href="https://docs.rs/osynic_midi" target="_blank"><img src="https://img.shields.io/docsrs/osynic_midi?style=flat-square"/></a>
  <a href="https://github.com/osynicite/osynic_midi" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green?style=flat-square"/></a>
  <a href="https://github.com/osynicite/osynic_midi" target="_blank"><img src="https://img.shields.io/github/stars/osynicite/osynic_midi?style=flat-square"/></a>
</p>

<hr>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="#-examples">Examples</a> •
  <a href="README.md">中文</a>
</p>

---

## 📖 About This Project

**OsynicMIDI** is a lightweight, efficient MIDI to keyboard mapping utility written in Rust. It allows you to control keyboard input on your computer using any MIDI device, such as electronic keyboards, drum pads, or MIDI controllers.

Whether you're a gamer, music producer, or just want to creatively use MIDI devices, OsynicMIDI provides you with a simple yet powerful solution.

### Why Choose OsynicMIDI?

✅ **Simple & Easy** - Get started in 5 seconds, no complex configuration needed  
✅ **Powerful & Flexible** - Support multiple mapping modes and custom configurations  
✅ **Modern Interface** - Interactive menu with arrow keys and Enter button  
✅ **Library + CLI** - Use as a standalone tool or embed in your Rust project  
✅ **Reliable & Stable** - Well-tested and production-ready  
✅ **Open Source & Free** - MIT License, completely open source  

---

## ⚡ Quick Start

### 1️⃣ Install

```bash
cargo install osynic_midi
```

Or build from source:
```bash
git clone https://github.com/osynicite/osynic_midi
cd osynic_midi
cargo build --release
```

### 2️⃣ Run

```bash
osynic-midi start
```

### 3️⃣ Select

Use arrow keys `↑` `↓` to select your configuration and device, press `Enter` to confirm!

That's it! Now you can play keyboard keys with your MIDI device 🎉

---

## ✨ Features

### 🎹 MIDI Mapping
- **Two Mapping Modes**
  - **Notes Mode**: Map each MIDI note to a specific keyboard key
  - **Octaves Mode**: Automatically map by octave and pitch

### 🎯 Smart Device Management
- Auto-discover all connected MIDI devices
- Reliable device connection by name (not volatile index)
- Support for multiple MIDI devices

### ⚙️ Flexible Configuration
- Human-readable JSON format configuration
- Preset configurations ready out-of-the-box
- Easy custom mapping creation
- Support for velocity threshold filtering

### 🖥️ Modern CLI
- Interactive menu selection (similar to Vite)
- Intuitive arrow key navigation
- Non-interactive mode for scripts and automation
- Complete command-line help system

### 📚 Comprehensive Documentation
- Complete API documentation
- Detailed configuration guide
- Rich usage examples
- Architecture and design documentation

---

## 📦 Installation

### Using Cargo (Recommended)

```bash
cargo install osynic_midi
```

### Build from Source

```bash
git clone https://github.com/osynicite/osynic_midi.git
cd osynic_midi
cargo build --release
```

The compiled binary will be located at:
- **Debug build**: `target/debug/osynic-midi.exe`
- **Release build**: `target/release/osynic-midi.exe`

### System Requirements

- **Rust**: 1.85.0 or higher
- **OS**: Windows, Linux, or macOS
- **MIDI Device**: At least one MIDI input device (optional - virtual MIDI ports supported for testing)

---

## 🚀 Usage Guide

### Basic Commands

```bash
# Quick Start
osynic-midi

# List all connected MIDI devices
osynic-midi list-devices

# List all available configuration files
osynic-midi list-configs

# Start interactive MIDI mapping
osynic-midi start

# Create a new MIDI mapping configuration file (interactive)
osynic-midi create

# Load a configuration file from a local path
osynic-midi load-local

# Start with specific configuration
osynic-midi start -c configs/my_config.json

# Specify mapping mode (notes or octaves)
osynic-midi start -m notes

# Specify both config and mode (non-interactive)
osynic-midi start -c configs/my_config.json -m notes

# Load from local path with mode specified
osynic-midi load-local -c /path/to/config.json -m notes
```

### Interactive Menu

When you run `osynic-midi start`, you'll see selection menus:

```
? Select configuration file:
❯ midi_config.json
  midi_config_10ka.json
  midi_config_10kab.json
  ...
```

**Controls**:
- `↑` / `↓` - Navigate up/down
- `Enter` - Confirm selection
- `Ctrl+C` - Cancel

### Workflow

1. **Check Your Device**
   ```bash
   osynic-midi list-devices
   ```
   Ensure your MIDI device is recognized

2. **Choose Configuration Method**
   
   **Option A: Use Existing Configuration (Recommended for beginners)**
   ```bash
   osynic-midi list-configs
   osynic-midi start
   ```
   
   **Option B: Create New Configuration (Recommended for custom setup)**
   ```bash
   osynic-midi create
   ```
   Follow the interactive prompts to create your mapping
   
   **Option C: Load Local Configuration File**
   ```bash
   osynic-midi load-local
   ```
   Specify the full path to your configuration file (works anywhere)

3. **Start Mapping**
   ```bash
   osynic-midi start
   ```
   Follow the interactive prompts to set up

4. **Start Using**
   Now you can use your MIDI device to play keyboard keys!

---

## 📚 Documentation

Full documentation is available in the `docs/` directory:

| Document                                              | Content                                   |
| ----------------------------------------------------- | ----------------------------------------- |
| [Getting Started](docs/01-getting-started.md)         | 5-minute quick start guide                |
| [CLI Complete Guide](docs/02-cli-guide.md)            | Detailed command-line tool documentation  |
| [Library API Docs](docs/03-library-api.md)            | Complete API reference for Rust library   |
| [Architecture](docs/04-architecture.md)               | Project structure and design explanation  |
| [Configuration Guide](docs/05-configuration-guide.md) | Detailed JSON configuration documentation |
| [Changelog](docs/06-changelog.md)                     | Version history and updates               |
| [Troubleshooting](docs/07-troubleshooting.md)         | FAQ and solutions                         |

---

## 💡 Examples

### Example 1: Quick Start
```bash
osynic-midi start -c configs/midi_config.json -m notes
```

### Example 2: Using in a Library
```rust
use osynic_midi::{Config, MappingMode, start_mapping};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load("configs/midi_config.json")?;
    
    // Start MIDI mapping
    start_mapping(
        "configs/midi_config.json".to_string(),
        "X8III".to_string(),
        MappingMode::Notes,
    ).await?;
    
    Ok(())
}
```

See the `examples/` directory for more examples.

---

## 🎯 Use Cases

### 🎮 Gamers
- Control rhythm games with MIDI keyboard (Osu!mania, etc.)
- Create professional gaming peripheral mappings

### 🎵 Music Producers
- Control your DAW with MIDI controllers
- Create custom keyboard shortcut mappings

### 💻 Developers
- Integrate as a library into Rust projects
- Build professional MIDI applications

### 🏠 Enthusiasts
- Creative MIDI device usage
- Application automation and control

---

## 🔧 Configuration Examples

### Osu!Mania Style

```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 1,
  "octaves": {},
  "note_mappings": {
    "36": "A", "37": "S", "38": "D", "39": "F",
    "40": "G", "41": "H", "42": "J", "43": "K"
  }
}
```

### Piano Keyboard Style

```json
{
  "mapping_mode": "octaves",
  "velocity_threshold": 0,
  "octaves": {
    "3": {
      "C": "A", "C#/Db": "W", "D": "S", "D#/Eb": "E",
      "E": "D", "F": "R", "F#/Gb": "F", "G": "T",
      "G#/Ab": "G", "A": "Y", "A#/Bb": "H", "B": "U"
    }
  },
  "note_mappings": {}
}
```

See [Configuration Guide](docs/05-configuration-guide.md) for more configuration examples.

---

## 🤝 Contributing

Contributions are welcome! You can help in many ways:

- 🐛 Report bugs
- 💡 Suggest new features
- 📝 Improve documentation
- 🔧 Submit code fixes

Please reach out via GitHub Issues or Pull Requests.

---

## ❓ FAQ

### Q: Which operating systems are supported?
A: Windows, Linux, and macOS are all supported.

### Q: Can I use it without a MIDI device?
A: Yes, you can use virtual MIDI ports like loopMIDI (Windows) for testing.

### Q: Can I use multiple MIDI devices?
A: Currently one device at a time, but you can switch quickly.

### Q: How do I create a custom configuration?
A: The JSON format is very simple. See [Configuration Guide](docs/05-configuration-guide.md).

### Q: Can I use it in scripts?
A: Yes! Use the `-c` and `-m` parameters for non-interactive mode.

See [Troubleshooting Guide](docs/07-troubleshooting.md) for more questions.

---

## 📊 Project Status

- ✅ Stable release
- ✅ Production-ready
- ✅ Complete documentation
- ✅ Actively maintained

---

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

Thanks to the following projects that make OsynicMIDI possible:
- [midir](https://github.com/Boddlnagg/midir) - MIDI support
- [enigo](https://github.com/enigo-rs/enigo) - Keyboard simulation
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [inquire](https://github.com/mikaelmello/inquire) - Interactive menus

---

## 📮 Contact

- 📧 Email: [zoneherobrine@gmail.com](mailto:zoneherobrine@gmail.com)
- 🐙 GitHub: [@osynicite](https://github.com/osynicite)
- 🌐 Project: [osynic_midi](https://github.com/osynicite/osynic_midi)

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/osynicite">Hako Chest</a>
  <br><br>
  <a href="README.md">🌏 中文版本</a>
</p>
| -------------- | ----- | ----------- | ------------------------------ |
| --beatmapsets  | -b    | -           | Path to native mode JSON file  |
| --osynic-songs | -n    | -           | Path to Osynic mode JSON file  |
| --source       | -s    | SayoApi     | osu! beatmap download source   |
| --username     | -u    | -           | osu! account (only for OsuDirect/OsuApiV2) |
| --password     | -p    | -           | osu! password (only for OsuDirect/OsuApiV2) |
| --output       | -o    | beatmapsets | Download directory (auto-created) |
| --concurrency  | -c    | 4           | Download concurrency (1-16)    |
| --help         | -h    | -           | Display help information        |

## 📥 Supported osu! Download Sources

1. **OsuDirect**: Official osu! beatmap download source (osu username and password required, URL parameters)
2. **OsuApiV2**(Not available yet): osu!lazer beatmap download source (osu username and password required, Basic authentication)
3. **SayoApi** (default): Sayobot beatmap download source (no login required)
4. **ChimuApi**: Chimu.moe beatmap download source (no login required)

## 📌 Notes

1. Video download adaptation (no_video) is not yet implemented, and related options will be ignored
2. Download file naming follows the `{{filename}}` naming rule
3. Interrupting the download process with `Ctrl+C` and then rerunning will resume the download
4. It is recommended to use a stable network connection for the best experience

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

- Follow the official Rust coding style
- Add test cases for new features
- Run `cargo fmt` and `cargo clippy` before submitting

## 📜 License

This project is open-sourced under the [MIT License](LICENSE). Please respect the original author's copyright. When using osu! related resources, please follow the [osu! community guidelines](https://osu.ppy.sh/wiki/zh/Legal).
