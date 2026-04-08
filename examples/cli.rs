use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::path::Path;
use serde::{ Deserialize, Serialize };
use std::fs;
use midir::MidiInput;
use enigo::{ Direction::{ Press, Release }, Enigo, Settings, Key, Keyboard };
use clap::{ Parser, Subcommand };
use inquire::Select;

#[derive(Parser)]
#[command(name = "osynic-midi")]
#[command(about = "MIDI to Keyboard mapper for Osynic", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,

    /// Mapping mode: octaves or notes
    #[arg(short, long)]
    mode: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// List available MIDI input devices
    ListDevices,
    /// List available configuration files
    ListConfigs,
    /// Start MIDI mapping with interactive setup
    Start {
        /// Configuration file path (will prompt if not provided)
        #[arg(short, long)]
        config: Option<String>,

        /// Mapping mode: octaves or notes (will prompt if not provided)
        #[arg(short, long)]
        mode: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum MappingMode {
    Octaves,
    Notes,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    mapping_mode: Option<MappingMode>,
    octaves: HashMap<String, HashMap<String, String>>,
    velocity_threshold: u8,
    note_mappings: HashMap<u8, String>,
}

impl Config {
    fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    fn note_to_pitch(note: u8) -> (u8, String) {
        let octave = note / 12 - 1;
        let pitch = match note % 12 {
            0 => "C",
            1 => "C#/Db",
            2 => "D",
            3 => "D#/Eb",
            4 => "E",
            5 => "F",
            6 => "F#/Gb",
            7 => "G",
            8 => "G#/Ab",
            9 => "A",
            10 => "A#/Bb",
            11 => "B",
            _ => unreachable!(),
        };
        (octave, pitch.to_string())
    }

    fn get_key_for_note(&self, note: u8, mode: &MappingMode) -> Option<Key> {
        let key_str = match mode {
            MappingMode::Notes => self.note_mappings.get(&note).cloned(),
            MappingMode::Octaves => {
                let (octave, pitch) = Self::note_to_pitch(note);
                self.octaves
                    .get(&octave.to_string())
                    .and_then(|octave_map| octave_map.get(&pitch).cloned())
            }
        };

        key_str.and_then(|key_str| {
            match key_str.as_str() {
                "," => Some(Key::Unicode(',')),
                "." => Some(Key::Unicode('.')),
                "/" => Some(Key::Unicode('/')),
                ";" => Some(Key::Unicode(';')),
                "'" => Some(Key::Unicode('\'')),
                "[" => Some(Key::Unicode('[')),
                "]" => Some(Key::Unicode(']')),
                "\\" => Some(Key::Unicode('\\')),
                "-" => Some(Key::Unicode('-')),
                "=" => Some(Key::Unicode('=')),
                "Space" => Some(Key::Space),
                "Left" => Some(Key::LeftArrow),
                "Right" => Some(Key::RightArrow),
                "A" => Some(Key::A),
                "B" => Some(Key::B),
                "C" => Some(Key::C),
                "D" => Some(Key::D),
                "E" => Some(Key::E),
                "F" => Some(Key::F),
                "G" => Some(Key::G),
                "H" => Some(Key::H),
                "I" => Some(Key::I),
                "J" => Some(Key::J),
                "K" => Some(Key::K),
                "L" => Some(Key::L),
                "M" => Some(Key::M),
                "N" => Some(Key::N),
                "O" => Some(Key::O),
                "P" => Some(Key::P),
                "Q" => Some(Key::Q),
                "R" => Some(Key::R),
                "S" => Some(Key::S),
                "T" => Some(Key::T),
                "U" => Some(Key::U),
                "V" => Some(Key::V),
                "W" => Some(Key::W),
                "X" => Some(Key::X),
                "Y" => Some(Key::Y),
                "Z" => Some(Key::Z),
                "RAlt" => Some(Key::Alt),
                _ => None,
            }
        })
    }
}

#[derive(Debug)]
enum KeyEvent {
    NoteOn(u8, u8), // (note, velocity)
    NoteOff(u8), // note
}

struct KeyboardMapper {
    config: Config,
    enigo: Arc<Mutex<Enigo>>,
    mode: MappingMode,
}

impl KeyboardMapper {
    fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self {
        Self { config, enigo, mode }
    }

    fn handle_event(&self, event: KeyEvent) -> Result<(), Box<dyn Error>> {
        if let Ok(mut enigo_guard) = self.enigo.lock() {
            match event {
                KeyEvent::NoteOn(note, velocity) => {
                    if velocity >= self.config.velocity_threshold {
                        if let Some(key) = self.config.get_key_for_note(note, &self.mode) {
                            enigo_guard.key(key, Press)?;
                        }
                    }
                }
                KeyEvent::NoteOff(note) => {
                    if let Some(key) = self.config.get_key_for_note(note, &self.mode) {
                        enigo_guard.key(key, Release)?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// List available configuration files in the configs directory
fn list_configs() -> Result<Vec<String>, Box<dyn Error>> {
    let config_dir = "configs";
    let mut configs = Vec::new();

    if !Path::new(config_dir).exists() {
        println!("Configuration directory '{}' does not exist", config_dir);
        return Ok(configs);
    }

    for entry in fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                configs.push(filename.to_string());
            }
        }
    }

    configs.sort();
    Ok(configs)
}

/// Display available configuration files
fn display_configs() -> Result<(), Box<dyn Error>> {
    println!("\nAvailable configuration files:");
    let configs = list_configs()?;

    if configs.is_empty() {
        println!("  No configuration files found in 'configs' directory");
        return Ok(());
    }

    for (i, config) in configs.iter().enumerate() {
        println!("  {}: {}", i, config);
    }

    Ok(())
}

/// List available MIDI input devices
fn display_devices() -> Result<(), Box<dyn Error>> {
    let midi_in = MidiInput::new("osynic-midi")?;
    let in_ports = midi_in.ports();

    println!("\nAvailable MIDI input devices:");
    if in_ports.is_empty() {
        println!("  No MIDI input devices found");
        return Ok(());
    }

    for (i, p) in in_ports.iter().enumerate() {
        println!("  {}: {}", i, midi_in.port_name(p)?);
    }

    Ok(())
}

/// Prompt user to select a configuration file
fn select_config() -> Result<String, Box<dyn Error>> {
    let configs = list_configs()?;
    if configs.is_empty() {
        return Err("No configuration files available".into());
    }

    println!();
    let selection = Select::new("Select configuration file:", configs.clone()).prompt()?;
    let selected = format!("configs/{}", selection);
    Ok(selected)
}

/// Prompt user to select a MIDI input device
fn select_device() -> Result<usize, Box<dyn Error>> {
    let midi_in = MidiInput::new("osynic-midi")?;
    let in_ports = midi_in.ports();

    if in_ports.is_empty() {
        return Err("No MIDI input devices found".into());
    }

    let mut device_names = Vec::new();
    for p in in_ports.iter() {
        device_names.push(midi_in.port_name(p)?);
    }

    println!();
    let selection = Select::new("Select MIDI input device:", device_names.clone()).prompt()?;

    let device_idx = device_names
        .iter()
        .position(|d| d == &selection)
        .ok_or("Device selection failed")?;

    Ok(device_idx)
}

/// Prompt user to select mapping mode
fn select_mode() -> Result<MappingMode, Box<dyn Error>> {
    let modes = vec!["Notes (individual note to key mapping)", "Octaves (octave-based mapping)"];

    println!();
    let selection = Select::new("Select mapping mode:", modes).prompt()?;

    let mode = match selection {
        s if s.starts_with("Notes") => MappingMode::Notes,
        s if s.starts_with("Octaves") => MappingMode::Octaves,
        _ => MappingMode::Notes,
    };

    Ok(mode)
}

/// Start the MIDI to keyboard mapping
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Some(Commands::ListDevices) => { display_devices() }
        Some(Commands::ListConfigs) => { display_configs() }
        Some(Commands::Start { config, mode }) => { start_mapping(config, mode).await }
        None => {
            // Default: start interactive setup
            start_mapping(args.config, args.mode).await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn start_mapping(
    config_path: Option<String>,
    mode_arg: Option<String>
) -> Result<(), Box<dyn Error>> {
    // Select configuration file
    let config_path = match config_path {
        Some(path) => path,
        None => select_config()?,
    };

    // Load configuration
    println!("\nLoading configuration from: {}", config_path);
    let mut config = Config::load(&config_path)?;

    // Determine mapping mode
    let mode = match mode_arg {
        Some(m) => {
            match m.to_lowercase().as_str() {
                "octaves" => MappingMode::Octaves,
                "notes" => MappingMode::Notes,
                _ => select_mode()?,
            }
        }
        None => {
            config.mapping_mode
                .take()
                .unwrap_or_else(|| select_mode().unwrap_or(MappingMode::Notes))
        }
    };

    println!("Using mapping mode: {:?}", mode);

    // Initialize Enigo
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mapper = Arc::new(KeyboardMapper::new(config, Arc::clone(&enigo), mode));

    // Create channel for MIDI events
    let (tx, mut rx) = mpsc::channel::<KeyEvent>(32);

    // Select MIDI input device
    let device_idx = select_device()?;

    // Connect to MIDI device
    let midi_in = MidiInput::new("osynic-midi")?;
    let in_ports = midi_in.ports();
    let in_port = &in_ports[device_idx];
    let device_name = midi_in.port_name(in_port)?;

    println!("\nOpening MIDI connection: {}", device_name);

    let tx_clone = tx.clone();
    let _conn_in = midi_in.connect(
        in_port,
        "osynic-midi",
        move |_stamp, message, _| {
            if message.len() == 3 {
                let status = message[0];
                let note = message[1];
                let velocity = message[2];

                let event = if status == 0x90 && velocity > 0 {
                    Some(KeyEvent::NoteOn(note, velocity))
                } else if status == 0x80 || (status == 0x90 && velocity == 0) {
                    Some(KeyEvent::NoteOff(note))
                } else {
                    None
                };

                if let Some(event) = event {
                    let _ = tx_clone.try_send(event);
                }
            }
        },
        ()
    )?;

    // Create event handler task
    let event_handler = tokio::spawn({
        let mapper = Arc::clone(&mapper);
        async move {
            while let Some(event) = rx.recv().await {
                if let Err(e) = mapper.handle_event(event) {
                    eprintln!("Error handling event: {}", e);
                }
            }
        }
    });

    println!("\n✓ MIDI mapping is active!");
    println!("Press Ctrl+C to stop, or use the keyboard/MIDI device to interact.\n");

    // Wait indefinitely - user needs to press Ctrl+C to exit
    std::future::pending::<()>().await;

    println!("Shutting down...");
    drop(tx);
    event_handler.abort();

    Ok(())
}
