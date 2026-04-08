/// Example: Using osynic_midi as a library
/// 
/// This example demonstrates how external applications can use
/// the osynic_midi library to implement MIDI mapping functionality
/// without relying on the CLI.

use osynic_midi::{Config, MappingMode, KeyboardMapper};
use enigo::{Enigo, Settings};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Osynic MIDI Library Example ===\n");

    // Example 1: Load a configuration
    println!("1. Loading configuration...");
    let config = Config::load("configs/midi_config.json")?;
    println!("   ✓ Config loaded successfully");
    println!("   - Velocity threshold: {}", config.velocity_threshold);
    println!("   - Note mappings available: {}", config.note_mappings.len());

    // Example 2: Create a keyboard mapper
    println!("\n2. Creating keyboard mapper...");
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let _mapper = KeyboardMapper::new(config, Arc::clone(&enigo), MappingMode::Notes);
    println!("   ✓ Mapper created successfully");

    // Example 3: List available configurations
    println!("\n3. Discovering configurations...");
    let configs = osynic_midi::list_configs()?;
    println!("   ✓ Found {} configuration files:", configs.len());
    for (i, config_name) in configs.iter().enumerate().take(5) {
        println!("     {}. {}", i + 1, config_name);
    }
    if configs.len() > 5 {
        println!("     ... and {} more", configs.len() - 5);
    }

    // Example 4: List available MIDI devices
    println!("\n4. Discovering MIDI devices...");
    let devices = osynic_midi::list_midi_devices()?;
    println!("   ✓ Found {} MIDI devices:", devices.len());
    for (i, device) in devices.iter().enumerate() {
        println!("     {}. {}", i + 1, device);
    }

    // Example 5: Using interactive functions
    println!("\n5. Interactive selection functions available:");
    println!("   - osynic_midi::select_config()   - Select configuration file");
    println!("   - osynic_midi::select_device()   - Select MIDI device");
    println!("   - osynic_midi::select_mode()     - Select mapping mode");

    // Example 6: Using the mapper
    println!("\n6. Handling MIDI events:");
    println!("   - Create KeyEvent::NoteOn(note, velocity)");
    println!("   - Create KeyEvent::NoteOff(note)");
    println!("   - Call mapper.handle_event(event)");

    // Example 7: Starting MIDI mapping
    println!("\n7. Programmatic control (without CLI):");
    println!("   - Use osynic_midi::start_mapping() function");
    println!("   - Pass config path, device index, and mapping mode");
    println!("   - Async function that runs the entire MIDI mapping");

    println!("\n=== Library is ready for integration! ===");
    println!("\nKey modules available:");
    println!("  • core:        Config, MappingMode, KeyboardMapper, KeyEvent");
    println!("  • discovery:   list_configs, list_midi_devices, display_*");
    println!("  • interactive: select_config, select_device, select_mode");
    println!("  • mapper:      start_mapping (async function)");

    Ok(())
}
