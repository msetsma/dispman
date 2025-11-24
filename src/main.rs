use clap::{Parser, Subcommand};
use display_cli::{config::{Config, Profile}, display, error::DisplayError};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "dispman")]
#[command(about = "A CLI tool for controlling monitor settings via DDC/CI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect available displays
    Detect {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Get capabilities of a display
    Capabilities {
        /// Display ID (index)
        #[arg(short, long)]
        display: Option<usize>,
    },
    /// Get a VCP feature value
    Get {
        /// Feature code (hex) or name (brightness, contrast, volume, input)
        feature: String,
        /// Display ID (index)
        #[arg(short, long)]
        display: Option<usize>,
    },
    /// Set a VCP feature value
    Set {
        /// Feature code (hex) or name (brightness, contrast, volume, input)
        feature: String,
        /// Value to set
        value: u32,
        /// Display ID (index)
        #[arg(short, long)]
        display: Option<usize>,
    },
    /// Manage profiles
    Profile {
        #[command(subcommand)]
        command: ProfileCommands,
    },
    /// Inspect all settings for a display (useful for creating profiles)
    Inspect {
        /// Display ID (index)
        #[arg(short, long)]
        display: Option<usize>,
    },
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// Save current settings as a profile
    Save {
        /// Profile name
        name: String,
    },
    /// Load/Apply a profile
    Load {
        /// Profile name
        name: String,
    },
    /// List available profiles
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect { json } => {
            let displays = display::enumerate_displays()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&displays)?);
            } else {
                for d in displays {
                    println!("Display {}: {} (Handle: {:?})", d.id, d.name, d.handle);
                }
            }
        }
        Commands::Capabilities { display } => {
            let displays = display::enumerate_displays()?;
            let target = get_display(&displays, display)?;
            let caps = target.capabilities()?;
            println!("{}", caps);
        }
        Commands::Get { feature, display } => {
            let displays = display::enumerate_displays()?;
            let target = get_display(&displays, display)?;
            let code = parse_feature(&feature)?;
            let value = target.get_vcp_feature(code)?;
            println!("Display {}: {} = {} (0x{:X})", target.id, feature, value, value);
        }
        Commands::Set { feature, value, display } => {
            let displays = display::enumerate_displays()?;
            let target = get_display(&displays, display)?;
            let code = parse_feature(&feature)?;
            target.set_vcp_feature(code, value)?;
            println!("Set {} to {}", feature, value);
        }
        Commands::Profile { command } => match command {
            ProfileCommands::Save { name } => {
                let displays = display::enumerate_displays()?;
                let mut config = Config::load()?;
                let mut settings = HashMap::new();

                for d in displays {
                    // Save common settings
                    let mut display_settings = Vec::new();
                    for code in [0x10, 0x12, 0x60, 0x62] { // Brightness, Contrast, Input, Volume
                        if let Ok(val) = d.get_vcp_feature(code) {
                            display_settings.push((code, val));
                        }
                    }
                    settings.insert(d.name.clone(), display_settings);
                }

                config.save_profile(name.clone(), Profile { settings });
                config.save()?;
                println!("Profile '{}' saved.", name);
            }
            ProfileCommands::Load { name } => {
                let config = Config::load()?;
                if let Some(profile) = config.get_profile(&name) {
                    let displays = display::enumerate_displays()?;
                    for d in displays {
                        if let Some(settings) = profile.settings.get(&d.name) {
                            for (code, value) in settings {
                                if let Err(e) = d.set_vcp_feature(*code, *value) {
                                    eprintln!("Failed to set feature 0x{:X} on display {}: {}", code, d.id, e);
                                }
                            }
                        }
                    }
                    println!("Profile '{}' loaded.", name);
                } else {
                    eprintln!("Profile '{}' not found.", name);
                }
            }
            ProfileCommands::List => {
                let config = Config::load()?;
                for name in config.profiles.keys() {
                    println!("{}", name);
                }
            }
        },
        Commands::Inspect { display } => {
            let displays = display::enumerate_displays()?;
            let target = get_display(&displays, display)?;
            println!("Inspecting Display {}: {}", target.id, target.name);
            
            // Common VCP codes to check
            let codes = vec![
                (0x10, "Brightness"),
                (0x12, "Contrast"),
                (0x60, "Input Source"),
                (0x62, "Volume"),
                (0xD6, "Power Mode"),
            ];

            for (code, name) in codes {
                match target.get_vcp_feature(code) {
                    Ok(val) => println!("{}: {} (0x{:X})", name, val, val),
                    Err(_) => println!("{}: Not supported", name),
                }
            }
        }
    }

    Ok(())
}

fn get_display(displays: &[display::Display], id: Option<usize>) -> Result<&display::Display, DisplayError> {
    if displays.is_empty() {
        return Err(DisplayError::MonitorNotFound("No displays found".to_string()));
    }
    
    if let Some(id) = id {
        displays.iter().find(|d| d.id == id).ok_or_else(|| DisplayError::MonitorNotFound(format!("Display {} not found", id)))
    } else {
        // Default to first display
        Ok(&displays[0])
    }
}

fn parse_feature(feature: &str) -> Result<u8, DisplayError> {
    match feature.to_lowercase().as_str() {
        "brightness" => Ok(0x10),
        "contrast" => Ok(0x12),
        "volume" => Ok(0x62),
        "input" => Ok(0x60),
        "power" => Ok(0xD6),
        s => {
            if s.starts_with("0x") {
                u8::from_str_radix(&s[2..], 16).map_err(|_| DisplayError::FeatureNotSupported(format!("Invalid hex code: {}", s)))
            } else {
                s.parse::<u8>().map_err(|_| DisplayError::FeatureNotSupported(format!("Unknown feature: {}", s)))
            }
        }
    }
}
