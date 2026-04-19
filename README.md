# Display Manager (dispman)

[![Release](https://img.shields.io/github/v/release/msetsma/dispman?logo=github)](https://github.com/msetsma/dispman/releases/latest)
[![Release workflow](https://img.shields.io/github/actions/workflow/status/msetsma/dispman/release.yml?label=release%20build&logo=github)](https://github.com/msetsma/dispman/actions/workflows/release.yml)
[![Downloads](https://img.shields.io/github/downloads/msetsma/dispman/total?logo=github)](https://github.com/msetsma/dispman/releases)
[![License: MIT](https://img.shields.io/github/license/msetsma/dispman)](LICENSE)
[![Homebrew tap](https://img.shields.io/badge/homebrew-msetsma%2Fdispman-orange?logo=homebrew)](https://github.com/msetsma/homebrew-dispman)

A lightweight CLI tool for controlling monitor/display settings via DDC/CI, with primary focus on switching display inputs programmatically.

## Platform Support

| Platform | Status  | Backend                                          |
|----------|---------|--------------------------------------------------|
| Windows  | ✅       | Win32 `GetVCPFeatureAndVCPFeatureReply` / `SetVCPFeature` |
| macOS    | ✅       | [`ddc-macos`](https://crates.io/crates/ddc-macos) (IOKit, Intel + Apple Silicon) |
| Linux    | planned | [`ddc-i2c`](https://crates.io/crates/ddc-i2c) via `/dev/i2c-*` |

**macOS notes:** No Administrator privileges required. On Apple Silicon, DDC/CI works over USB-C/DisplayPort alt mode; the built-in HDMI port on entry-level M1/M2 Macs is not supported by the underlying IOKit APIs.

## Goal

Enable programmatic control of monitor settings, particularly input source switching, without requiring physical button presses or OSD navigation.

**Primary Goal:** Change monitor input source (HDMI1, HDMI2, DisplayPort, etc.)

**Secondary Goals:**
- Adjust brightness/contrast
- Control other DDC/CI-accessible settings
- Support multiple monitors

## Requirements

- Written in Rust
- Windows support (primary target)
- macOS support (future, separate implementation if needed)
- User can select which display to interact with
- User can select which setting to interact with
- User can set or get values for settings

## CLI Design Philosophy

Following [clig.dev](https://clig.dev/) guidelines with emphasis on:
Use Windows DDC/CI APIs to communicate with monitors:
- `GetMonitorCapabilities()` - Query what monitor supports
- `GetVCPFeatureAndVCPFeatureReply()` - Read current values
- `SetVCPFeature()` - Write new values

These APIs communicate over the video cable using the DDC/CI protocol (I2C-based) to control monitor settings that would normally be accessed via OSD buttons.

### Key Concepts

- **VCP Codes** (Virtual Control Panel): Standardized codes for monitor settings
  - `0x60`: Input source
  - `0x10`: Brightness
  - `0x12`: Contrast
  - `0x62`: Audio volume
- **DDC/CI Protocol**: Display Data Channel / Command Interface
- **Capabilities String**: Monitor-reported list of supported features

## Known Limitations

- Not all monitors implement DDC/CI correctly
- Some USB-C hubs/docks don't pass DDC commands reliably
- Monitor firmware quality varies significantly
- Some displays require delays between commands

## Future Considerations

- macOS support may require different APIs (IOKit framework)
- Profile/preset system for quick switching between configurations
- Monitor-specific quirks/workarounds database

## Installation

### Homebrew (macOS)

```sh
brew tap msetsma/dispman
brew install dispman
```

### Shell installer (macOS / Linux)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/msetsma/dispman/releases/latest/download/dispman-installer.sh | sh
```

### PowerShell installer (Windows)

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/msetsma/dispman/releases/latest/download/dispman-installer.ps1 | iex"
```

### From source

```sh
cargo install --git https://github.com/msetsma/dispman
```

## Usage

### Running Locally

To run the tool locally, use `cargo run` followed by `--` to pass arguments to the CLI.

```powershell
cargo run -- <command> [options]
```

### Available Commands

#### Detect Displays
Finds all connected monitors that support DDC/CI.
```sh
dispman detect
# Output as JSON
dispman detect --json
```

#### Inspect Display
Shows a summary of common settings (Brightness, Contrast, Input, Volume, Power) for a specific display.
```sh
# Inspect the default (first) display
dispman inspect

# Inspect a specific display by ID (e.g., 1)
dispman inspect --display 1
```

#### Get a Setting
Read a specific value. You can use names like `brightness`, `contrast`, `volume`, `input`, or raw hex codes (e.g., `0x10`).
```sh
# Get brightness of default display
dispman get brightness

# Get contrast of display 1
dispman get contrast --display 1
```

#### Set a Setting
Change a value.
```sh
# Set brightness to 50%
dispman set brightness 50

# Set input source to HDMI1 (Commonly 0x11 or 17, but varies by monitor)
dispman set input 17 --display 1
```

#### Check Capabilities
Reads the raw capabilities string from the monitor.
```sh
dispman capabilities
```

#### Profiles
Save and load configurations.

Profiles are keyed by a stable display identifier (EDID-derived on macOS; Windows device path for now), so saved profiles apply to the same physical monitor across reboots. **Note:** this changed in the cross-platform refactor — profiles saved with earlier versions must be re-saved.

```sh
# List saved profiles
dispman profile list

# Save current settings of all monitors as "work"
dispman profile save work

# Load the "work" profile
dispman profile load work
```

### Troubleshooting
- **Administrator Privileges:** DDC/CI commands often require running the terminal as **Administrator** on Windows.
- **Monitor Support:** If commands fail, ensure "DDC/CI" is enabled in your monitor's OSD menu.

## Contributing

[To be added: contribution guidelines, bug reporting, etc.]

## License

MIT — see [LICENSE](LICENSE).