mod cli;

use clap::Parser;
use cli::{Cli, Commands, ProfileCommands};
use dispman::{
    backend,
    config::{Config, Profile},
    error::DisplayError,
};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect { json } => {
            let displays = backend::enumerate()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&displays)?);
            } else {
                for d in displays {
                    println!(
                        "Display {}: {} (stable id: {})",
                        d.id,
                        d.name(),
                        d.stable_id()
                    );
                }
            }
        }
        Commands::Capabilities { display } => {
            let mut displays = backend::enumerate()?;
            let target = select_display_mut(&mut displays, display)?;
            let caps_str = target.capabilities()?;
            let caps = dispman::capabilities::Capabilities::parse(&caps_str);
            println!("{}", caps);
        }
        Commands::Get { feature, display } => {
            let mut displays = backend::enumerate()?;
            let target = select_display_mut(&mut displays, display)?;
            let code = parse_feature(&feature)?;
            let value = target.get_vcp_feature(code)?;
            println!(
                "Display {}: {} = {} (0x{:X})",
                target.id, feature, value, value
            );
        }
        Commands::Set {
            feature,
            value,
            display,
        } => {
            let mut displays = backend::enumerate()?;
            let target = select_display_mut(&mut displays, display)?;
            let code = parse_feature(&feature)?;
            target.set_vcp_feature(code, value)?;
            println!("Set {} to {}", feature, value);
        }
        Commands::Profile { command } => match command {
            ProfileCommands::Save { name } => {
                let mut displays = backend::enumerate()?;
                let mut config = Config::load()?;
                let mut settings = HashMap::new();

                for d in displays.iter_mut() {
                    let mut display_settings = Vec::new();
                    for code in [0x10, 0x12, 0x60, 0x62] {
                        if let Ok(val) = d.get_vcp_feature(code) {
                            display_settings.push((code, val));
                        }
                    }
                    settings.insert(d.stable_id().to_string(), display_settings);
                }

                config.save_profile(name.clone(), Profile { settings });
                config.save()?;
                println!("Profile '{}' saved.", name);
            }
            ProfileCommands::Load { name } => {
                let config = Config::load()?;
                if let Some(profile) = config.get_profile(&name).cloned() {
                    let mut displays = backend::enumerate()?;
                    for d in displays.iter_mut() {
                        if let Some(settings) = profile.settings.get(d.stable_id()) {
                            for (code, value) in settings {
                                if let Err(e) = d.set_vcp_feature(*code, *value) {
                                    eprintln!(
                                        "Failed to set feature 0x{:X} on display {}: {}",
                                        code, d.id, e
                                    );
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
            let mut displays = backend::enumerate()?;
            let target = select_display_mut(&mut displays, display)?;
            println!("Inspecting Display {}: {}", target.id, target.name());

            let codes = [
                (0x10u8, "Brightness"),
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

fn select_display_mut(
    displays: &mut [backend::Display],
    id: Option<usize>,
) -> Result<&mut backend::Display, DisplayError> {
    if displays.is_empty() {
        return Err(DisplayError::MonitorNotFound(
            "No displays found".to_string(),
        ));
    }

    match id {
        Some(id) => displays
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| DisplayError::MonitorNotFound(format!("Display {} not found", id))),
        None => Ok(&mut displays[0]),
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
            if let Some(hex) = s.strip_prefix("0x") {
                u8::from_str_radix(hex, 16).map_err(|_| {
                    DisplayError::FeatureNotSupported(format!("Invalid hex code: {}", s))
                })
            } else {
                s.parse::<u8>().map_err(|_| {
                    DisplayError::FeatureNotSupported(format!("Unknown feature: {}", s))
                })
            }
        }
    }
}
