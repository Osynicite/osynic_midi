use std::error::Error;
use std::path::Path;
use std::fs;
use midir::MidiInput;

/// List available configuration files in the configs directory
pub fn list_configs() -> Result<Vec<String>, Box<dyn Error>> {
    let config_dir = "configs";
    let mut configs = Vec::new();

    if !Path::new(config_dir).exists() {
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

/// Get MIDI device names
pub fn list_midi_devices() -> Result<Vec<String>, Box<dyn Error>> {
    let midi_in = MidiInput::new("osynic-midi")?;
    let in_ports = midi_in.ports();

    let mut device_names = Vec::new();
    for p in in_ports.iter() {
        device_names.push(midi_in.port_name(p)?);
    }

    Ok(device_names)
}

/// Display available configuration files
pub fn display_configs() -> Result<(), Box<dyn Error>> {
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

/// Display available MIDI input devices
pub fn display_devices() -> Result<(), Box<dyn Error>> {
    println!("\nAvailable MIDI input devices:");
    let devices = list_midi_devices()?;

    if devices.is_empty() {
        println!("  No MIDI input devices found");
        return Ok(());
    }

    for (i, device) in devices.iter().enumerate() {
        println!("  {}: {}", i, device);
    }

    Ok(())
}
