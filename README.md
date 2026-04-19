# Display Manager (dispman)

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

Profiles are keyed by a stable display identifier (EDID-derived on macOS; Windows device path for now), so saved profiles apply to the same physical monitor across reboots. **Note:** this changed in the cross-platform refactor — profiles saved with earlier versions must be re-saved.

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

MIT — see [LICENSE](LICENSE).

---

## Implementation Notes for LLMs

### Releases

Releases are cut by pushing a SemVer tag (e.g. `v0.2.0`). The
`.github/workflows/release.yml` workflow (generated by
[`cargo-dist`](https://github.com/axodotdev/cargo-dist), configured in
`dist-workspace.toml`) builds per-target archives, creates the GitHub
Release, and publishes the Homebrew formula.

One-time prerequisites before the first tag:

1. Create the tap repository `msetsma/homebrew-dispman` on GitHub (empty).
2. Generate a GitHub Personal Access Token with `contents: write` on that
   tap repo, and add it to this repo's secrets as `HOMEBREW_TAP_TOKEN`.

To update the workflow after editing `dist-workspace.toml`, run `dist init --yes`
(or `dist generate`).

### Man page

`man/dispman.1` is regenerated by `build.rs` on every `cargo build`. The
clap-derived sections (NAME, SYNOPSIS, OPTIONS, SUBCOMMANDS) come from the
definitions in `src/cli.rs`; the prose-only sections (FEATURE NAMES, FILES,
EXAMPLES, PLATFORM NOTES, etc.) come from `man/footer.1`. Edit those two
sources, not the generated page.

### Project Structure
```
dispman/
├── src/
│   ├── main.rs              # CLI entry point, argument parsing
│   ├── cli.rs               # clap definitions (shared with build.rs)
│   ├── backend/
│   │   ├── mod.rs           # Display, DisplayInfo, DdcBackend trait, platform dispatch
│   │   ├── windows.rs       # Windows backend (Win32 DDC/CI)
│   │   └── macos.rs         # macOS backend (ddc-macos / IOKit)
│   ├── capabilities.rs      # MCCS capability string parser
│   ├── config.rs            # Profile storage (TOML)
│   ├── vcp.rs               # VCP code definitions and helpers
│   └── error.rs             # Error types and handling
├── man/
│   ├── dispman.1            # Generated by build.rs (committed, do not edit)
│   └── footer.1             # Hand-written prose sections appended to dispman.1
├── build.rs                 # Regenerates man/dispman.1 from src/cli.rs + footer
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