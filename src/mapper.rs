use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use std::sync::Mutex;
use midir::MidiInput;
use enigo::{ Enigo, Settings };

use crate::core::{ Config, MappingMode, KeyboardMapper, KeyEvent };

/// Start MIDI to keyboard mapping
pub async fn start_mapping(
    config_path: String,
    device_name: String,
    mode: MappingMode
) -> Result<(), Box<dyn Error>> {
    // Load configuration
    println!("\nLoading configuration from: {}", config_path);
    let config = Config::load(&config_path)?;

    println!("Using mapping mode: {:?}", mode);

    // Initialize Enigo
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mapper = Arc::new(KeyboardMapper::new(config, Arc::clone(&enigo), mode));

    // Create channel for MIDI events
    let (tx, mut rx) = mpsc::channel::<KeyEvent>(32);

    // Connect to MIDI device by name (not index, to avoid port list ordering issues)
    let midi_in = MidiInput::new("osynic-midi")?;
    let in_ports = midi_in.ports();
    let in_port = in_ports
        .iter()
        .find(|p| midi_in.port_name(p).unwrap_or_default() == device_name)
        .ok_or(format!("MIDI device '{}' not found", device_name))?;

    println!("Opening MIDI connection: {}", device_name);

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
