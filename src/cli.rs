// This module is intentionally free of crate-internal dependencies so that
// `build.rs` can `include!` it via a path without pulling in the rest of the
// crate. Only `clap` is used here.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dispman", version)]
#[command(about = "Control monitor settings via DDC/CI")]
#[command(long_about = "dispman reads and changes monitor settings over the DDC/CI protocol. \
It can switch input sources, adjust brightness and contrast, change volume, query a \
monitor's capabilities string, and save or restore groups of settings as named profiles.\n\n\
Monitors are addressed by a zero-based index assigned at enumeration time. \
If no display is given, commands operate on display 0.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Detect available displays
    #[command(long_about = "List all connected monitors that dispman can talk to. \
Each entry shows its numeric ID, a human-readable name, and the stable \
identifier used for profile lookup.")]
    Detect {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Get capabilities of a display
    #[command(long_about = "Query the MCCS capabilities string from a monitor and print a \
parsed summary including model, protocol, MCCS version, and supported VCP feature codes.")]
    Capabilities {
        /// Display ID (index)
        #[arg(short, long)]
        display: Option<usize>,
    },

    /// Get a VCP feature value
    #[command(long_about = "Read the current value of a VCP feature. FEATURE may be a \
well-known name (brightness, contrast, volume, input, power) or a raw code given as a \
hex literal (0xNN) or decimal integer. The value is printed in both decimal and hex.")]
    Get {
        /// Feature code (hex) or name (brightness, contrast, volume, input, power)
        feature: String,
        /// Display ID (index)
        #[arg(short, long)]
        display: Option<usize>,
    },

    /// Set a VCP feature value
    #[command(long_about = "Write a new value to a VCP feature. FEATURE accepts the same \
forms as `dispman get`. VALUE is a non-negative integer. Input-source codes are not \
standardized across monitor vendors; use `dispman capabilities` to discover the values \
your monitor accepts for code 0x60.")]
    Set {
        /// Feature code (hex) or name (brightness, contrast, volume, input, power)
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

    /// Inspect all settings for a display
    #[command(long_about = "Read and print the current values of the most common VCP \
features (brightness, contrast, input source, volume, power mode) for a single display. \
Features the monitor does not report are labelled `Not supported`.")]
    Inspect {
        /// Display ID (index)
        #[arg(short, long)]
        display: Option<usize>,
    },
}

#[derive(Subcommand)]
pub enum ProfileCommands {
    /// Save current settings as a profile
    #[command(long_about = "Capture the current brightness, contrast, input source, and \
volume of every connected monitor and store them under NAME in the user config file. \
If NAME already exists it is overwritten.")]
    Save {
        /// Profile name
        name: String,
    },

    /// Load/apply a profile
    #[command(long_about = "Apply a previously saved profile. For each monitor currently \
connected, dispman looks up the monitor's stable identifier in the profile and writes \
back the stored VCP values. Monitors not present in the profile are skipped; failures on \
individual features are reported on stderr but do not abort the command.")]
    Load {
        /// Profile name
        name: String,
    },

    /// List available profiles
    List,
}
