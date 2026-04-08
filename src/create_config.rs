use std::error::Error;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use inquire::{ Select, Text };
use crate::core::{ Config, MappingMode };

/// Interactive mode to create a new configuration file
/// Returns the path to the created configuration file
pub async fn create_config_interactive() -> Result<String, Box<dyn Error>> {
    println!("\n📝 Creating a new configuration file...\n");

    // Get filename
    let filename = Text::new("Configuration filename (without .json):").prompt()?;
    let config_filename = format!("configs/{}.json", filename);

    // Check if file already exists
    if Path::new(&config_filename).exists() {
        println!("\n⚠️  File already exists: {}", config_filename);
        return Err("Configuration file already exists".into());
    }

    // Select mapping mode
    println!();
    let modes = vec!["Notes", "Octaves"];
    let selected_mode = Select::new("Choose mapping mode:", modes).prompt()?;

    let mapping_mode = match selected_mode {
        "Notes" => MappingMode::Notes,
        "Octaves" => MappingMode::Octaves,
        _ => MappingMode::Notes,
    };

    // Get velocity threshold
    let velocity_threshold: u8 = loop {
        let input = Text::new("Velocity threshold (0-127, 0 means accept all):").prompt()?;

        match input.parse::<u8>() {
            Ok(v) if v <= 127 => {
                break v;
            }
            Ok(_) => println!("❌ Please enter a value between 0 and 127"),
            Err(_) => println!("❌ Please enter a valid number"),
        }
    };

    // Create configuration based on mode
    let config = if mapping_mode == MappingMode::Notes {
        create_notes_mode_config(velocity_threshold).await?
    } else {
        create_octaves_mode_config(velocity_threshold).await?
    };

    // Ensure configs directory exists
    fs::create_dir_all("configs")?;

    // Save configuration file
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_filename, config_json)?;

    println!("\n✅ Configuration file created successfully!");
    println!("📂 Saved to: {}", config_filename);
    println!("\n💡 You can now use it with: osynic-midi start -c {}", config_filename);

    Ok(config_filename)
}

/// Create configuration in Notes mode
async fn create_notes_mode_config(velocity_threshold: u8) -> Result<Config, Box<dyn Error>> {
    println!("\n🎹 Configuring Notes Mode");
    println!("(Mapping individual MIDI note numbers to keyboard keys)\n");

    let mut note_mappings = HashMap::new();

    loop {
        let note_input = Text::new(
            "Enter MIDI note number (0-127, or 'done' to finish):"
        ).prompt()?;

        if note_input.to_lowercase() == "done" || note_input.is_empty() {
            break;
        }

        match note_input.parse::<u8>() {
            Ok(note) if note <= 127 => {
                let key = Text::new(
                    &format!("Enter keyboard key for MIDI note {} (e.g., A, Space, Left):", note)
                ).prompt()?;

                note_mappings.insert(note, key);
                println!("✓ Note {} -> {}", note, note_mappings.get(&note).unwrap());
            }
            _ => println!("❌ Please enter a valid note number (0-127)"),
        }
    }

    Ok(Config {
        mapping_mode: Some(MappingMode::Notes),
        octaves: HashMap::new(),
        velocity_threshold,
        note_mappings,
    })
}

/// Create configuration in Octaves mode
async fn create_octaves_mode_config(velocity_threshold: u8) -> Result<Config, Box<dyn Error>> {
    println!("\n🎵 Configuring Octaves Mode");
    println!("(Mapping by octave and pitch)\n");

    let mut octaves = HashMap::new();
    let pitches = vec![
        "C",
        "C#/Db",
        "D",
        "D#/Eb",
        "E",
        "F",
        "F#/Gb",
        "G",
        "G#/Ab",
        "A",
        "A#/Bb",
        "B"
    ];

    loop {
        let octave_input = Text::new("Enter octave number (0-10, or 'done' to finish):").prompt()?;

        if octave_input.to_lowercase() == "done" || octave_input.is_empty() {
            break;
        }

        match octave_input.parse::<u8>() {
            Ok(octave) if octave <= 10 => {
                println!("\n🎹 Mapping octave {}:", octave);
                let mut octave_map = HashMap::new();

                for pitch in &pitches {
                    let key = Text::new(&format!("  {} {} -> ", octave, pitch)).prompt()?;
                    if !key.is_empty() {
                        octave_map.insert(pitch.to_string(), key);
                    }
                }

                if !octave_map.is_empty() {
                    octaves.insert(octave.to_string(), octave_map);
                    println!("✓ Octave {} configured", octave);
                }
            }
            _ => println!("❌ Please enter a valid octave number (0-10)"),
        }
    }

    Ok(Config {
        mapping_mode: Some(MappingMode::Octaves),
        octaves,
        velocity_threshold,
        note_mappings: HashMap::new(),
    })
}

/// Allow user to load configuration from a local file path
pub fn select_local_config() -> Result<String, Box<dyn Error>> {
    println!();
    let config_path = Text::new("Enter configuration file path (absolute or relative):").prompt()?;

    // Verify the file exists
    if !Path::new(&config_path).exists() {
        return Err(format!("Configuration file not found: {}", config_path).into());
    }

    // Verify it's a JSON file
    if !config_path.ends_with(".json") {
        return Err("Configuration file must be a .json file".into());
    }

    // Try to load and validate the configuration
    match crate::Config::load(&config_path) {
        Ok(_) => {
            println!("✅ Configuration loaded successfully!");
            Ok(config_path)
        }
        Err(e) => Err(format!("Failed to load configuration: {}", e).into()),
    }
}
