use clap::{ Parser, Subcommand };
use osynic_midi::{
    core::MappingMode,
    discovery::{ display_configs, display_devices },
    interactive::{ select_config, select_device, select_mode },
    start_mapping,
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
}

/// Start the MIDI to keyboard mapping
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Some(Commands::ListDevices) => display_devices(),
        Some(Commands::ListConfigs) => display_configs(),
        Some(Commands::Start { config, mode }) => { start_cli_mapping(config, mode).await }
        None => {
            // Default: start interactive setup
            start_cli_mapping(args.config, args.mode).await
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
