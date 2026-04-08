use clap::{ Parser, Subcommand };
use inquire::Select;
use osynic_midi::{
    core::MappingMode,
    discovery::{ display_configs, display_devices },
    interactive::{ select_config, select_device, select_mode },
    start_mapping,
    create_config_interactive,
    select_local_config,
};

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
    /// Create a new MIDI mapping configuration file
    #[command(about = "Create a new MIDI mapping configuration file", 
              long_about = "Interactively create a new MIDI mapping configuration file.\n\n\
                           You can choose between two modes:\n  \
                           • Notes Mode: Map individual MIDI note numbers to keyboard keys\n  \
                           • Octaves Mode: Map by octaves and pitch within each octave\n\n\
                           The config file will be saved to the configs/ directory.")]
    Create,
    /// Load a configuration file from a local path
    #[command(name = "load-local",
              about = "Load a configuration file from a local path",
              long_about = "Load a configuration file from anywhere on your file system.\n\n\
                           This allows you to use MIDI mapping configs stored outside the default configs/ directory.\n\n\
                           Usage:\n  \
                           • Interactive: osynic-midi load-local (will prompt for file path)\n  \
                           • With path: osynic-midi load-local -c /path/to/config.json\n  \
                           • With mode: osynic-midi load-local -c config.json -m notes")]
    LoadLocal {
        /// Configuration file path
        #[arg(short, long)]
        config: Option<String>,

        /// Mapping mode: octaves or notes (will prompt if not provided)
        #[arg(short, long)]
        mode: Option<String>,
    },
}

/// Start the MIDI to keyboard mapping
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Some(Commands::ListDevices) => display_devices(),
        Some(Commands::ListConfigs) => display_configs(),
        Some(Commands::Start { config, mode }) => { start_cli_mapping(config, mode).await }
        Some(Commands::Create) => { 
            match create_config_interactive().await {
                Ok(_config_filename) => {
                    // Config created successfully - just return Ok
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Some(Commands::LoadLocal { config, mode }) => { load_local_mapping(config, mode).await }
        None => {
            // Show main menu for configuration selection
            show_main_menu().await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn start_cli_mapping(
    config_path: Option<String>,
    mode_arg: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    // Select configuration file
    let config_path = match config_path {
        Some(path) => path,
        None => select_config()?,
    };

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
            // Try to read from config, otherwise prompt
            match osynic_midi::Config::load(&config_path) {
                Ok(mut config) =>
                    config.mapping_mode
                        .take()
                        .unwrap_or_else(|| { select_mode().unwrap_or(MappingMode::Notes) }),
                Err(_) => select_mode()?,
            }
        }
    };

    // Select MIDI input device
    let device_name = select_device()?;

    // Start MIDI mapping
    start_mapping(config_path, device_name, mode).await
}

async fn load_local_mapping(
    config_path: Option<String>,
    mode_arg: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from local path
    let config_path = match config_path {
        Some(path) => path,
        None => select_local_config()?,
    };

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
            // Try to read from config, otherwise prompt
            match osynic_midi::Config::load(&config_path) {
                Ok(mut config) =>
                    config.mapping_mode
                        .take()
                        .unwrap_or_else(|| { select_mode().unwrap_or(MappingMode::Notes) }),
                Err(_) => select_mode()?,
            }
        }
    };

    // Select MIDI input device
    let device_name = select_device()?;

    // Start MIDI mapping
    start_mapping(config_path, device_name, mode).await
}

/// Show main menu for configuration selection
async fn show_main_menu() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎵 MIDI to Keyboard Mapper - Configuration Menu\n");

    let options = vec![
        "Use existing configuration (from configs/ folder)",
        "Create a new configuration file",
        "Load configuration from a custom location",
    ];

    let selected = Select::new("Choose configuration method:", options).prompt()?;

    match selected {
        "Use existing configuration (from configs/ folder)" => {
            // Use default flow
            start_cli_mapping(None, None).await
        }
        "Create a new configuration file" => {
            // Show create config flow and get the created filename
            match create_config_interactive().await {
                Ok(config_filename) => {
                    // Ask if user wants to use it now
                    let use_now = Select::new(
                        "Do you want to use this configuration now?",
                        vec!["Yes", "No"]
                    ).prompt()?;
                    
                    if use_now == "Yes" {
                        println!("\n🚀 Starting MIDI mapping...");
                        start_cli_mapping(Some(config_filename), None).await
                    } else {
                        println!("\n👍 Configuration saved. You can use it anytime with:");
                        println!("   osynic-midi start -c {}", config_filename);
                        Ok(())
                    }
                }
                Err(e) => Err(e),
            }
        }
        "Load configuration from a custom location" => {
            // Use load-local flow
            load_local_mapping(None, None).await
        }
        _ => Err("Invalid selection".into()),
    }
}
