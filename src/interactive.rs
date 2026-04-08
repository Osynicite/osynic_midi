use crate::core::MappingMode;
use crate::discovery::list_configs;
use inquire::Select;
use std::error::Error;

/// Prompt user to select a configuration file
pub fn select_config() -> Result<String, Box<dyn Error>> {
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
pub fn select_device() -> Result<String, Box<dyn Error>> {
    use crate::discovery::list_midi_devices;

    let device_names = list_midi_devices()?;
    if device_names.is_empty() {
        return Err("No MIDI input devices found".into());
    }

    println!();
    let selection = Select::new("Select MIDI input device:", device_names.clone()).prompt()?;

    Ok(selection)
}

/// Prompt user to select mapping mode
pub fn select_mode() -> Result<MappingMode, Box<dyn Error>> {
    let modes = vec![
        "Notes (individual note to key mapping)",
        "Octaves (octave-based mapping)",
    ];

    println!();
    let selection = Select::new("Select mapping mode:", modes).prompt()?;

    let mode = match selection {
        s if s.starts_with("Notes") => MappingMode::Notes,
        s if s.starts_with("Octaves") => MappingMode::Octaves,
        _ => MappingMode::Notes,
    };

    Ok(mode)
}
