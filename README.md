# Display Manager (dispman)

A lightweight CLI tool for controlling monitor/display settings on Windows, with primary focus on switching display inputs programmatically.

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

## Usage

### Running Locally

To run the tool locally, use `cargo run` followed by `--` to pass arguments to the CLI.

```powershell
cargo run -- <command> [options]
```

### Available Commands

#### Detect Displays
Finds all connected monitors that support DDC/CI.
```powershell
cargo run -- detect
# Output as JSON
cargo run -- detect --json
```

#### Inspect Display
Shows a summary of common settings (Brightness, Contrast, Input, Volume, Power) for a specific display.
```powershell
# Inspect the default (first) display
cargo run -- inspect

# Inspect a specific display by ID (e.g., 1)
cargo run -- inspect --display 1
```

#### Get a Setting
Read a specific value. You can use names like `brightness`, `contrast`, `volume`, `input`, or raw hex codes (e.g., `0x10`).
```powershell
# Get brightness of default display
cargo run -- get brightness

# Get contrast of display 1
cargo run -- get contrast --display 1
```

#### Set a Setting
Change a value.
```powershell
# Set brightness to 50%
cargo run -- set brightness 50

# Set input source to HDMI1 (Commonly 0x11 or 17, but varies by monitor)
cargo run -- set input 17 --display 1
```

#### Check Capabilities
Reads the raw capabilities string from the monitor.
```powershell
cargo run -- capabilities
```

#### Profiles
Save and load configurations.
```powershell
# List saved profiles
cargo run -- profile list

# Save current settings of all monitors as "work"
cargo run -- profile save work

# Load the "work" profile
cargo run -- profile load work
```

### Troubleshooting
- **Administrator Privileges:** DDC/CI commands often require running the terminal as **Administrator** on Windows.
- **Monitor Support:** If commands fail, ensure "DDC/CI" is enabled in your monitor's OSD menu.

## Contributing

[To be added: contribution guidelines, bug reporting, etc.]

## License

[To be determined]

---

## Implementation Notes for LLMs

### Project Structure
```
dispman/
├── src/
│   ├── main.rs           # CLI entry point, argument parsing
│   ├── display.rs        # Display detection and enumeration
│   ├── ddc.rs           # DDC/CI protocol implementation
│   ├── vcp.rs           # VCP code definitions and helpers
│   └── error.rs         # Error types and handling
├── Cargo.toml
└── README.md
```

### Key Design Decisions

1. **Error Handling**: Use `anyhow` or `thiserror` for ergonomic error handling
2. **CLI Parsing**: Use `clap` with derive macros for clean argument parsing
3. **Output**: Default human-readable, `--json` for machine-readable via `serde_json`
4. **Windows API**: Use `windows` crate for FFI to Win32 APIs
5. **Async**: Not needed - DDC/CI operations are synchronous and fast

### VCP Code Reference (Common)

| Code | Setting | Values |
|------|---------|--------|
| 0x60 | Input Source | 0x0f=HDMI1, 0x10=HDMI2, 0x11=HDMI3, 0x0f=DP1, etc. |
| 0x10 | Brightness | 0-100 |
| 0x12 | Contrast | 0-100 |
| 0x62 | Volume | 0-100 |
| 0xD6 | Power Mode | 1=On, 4=Standby |

*Note: Input source codes are NOT standardized and vary by manufacturer*

### Testing Approach

- Unit tests for VCP code parsing/validation
- Integration tests with mock display responses
- Manual testing required for actual hardware interaction
- Document tested monitor models in README

### CLI Flag Standards

- `-h, --help`: Show help
- `-V, --version`: Show version
- `-d, --display <ID>`: Target specific display
- `--json`: Machine-readable output
- `-v, --verbose`: Verbose output
- `--debug`: Debug information