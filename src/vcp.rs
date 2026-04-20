use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VcpFeature {
    InputSource,
    Brightness,
    Contrast,
    Volume,
    PowerMode,
    Custom(u8),
}

impl VcpFeature {
    pub fn code(&self) -> u8 {
        match self {
            VcpFeature::InputSource => 0x60,
            VcpFeature::Brightness => 0x10,
            VcpFeature::Contrast => 0x12,
            VcpFeature::Volume => 0x62,
            VcpFeature::PowerMode => 0xD6,
            VcpFeature::Custom(c) => *c,
        }
    }

    pub fn from_code(code: u8) -> Self {
        match code {
            0x60 => VcpFeature::InputSource,
            0x10 => VcpFeature::Brightness,
            0x12 => VcpFeature::Contrast,
            0x62 => VcpFeature::Volume,
            0xD6 => VcpFeature::PowerMode,
            c => VcpFeature::Custom(c),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            VcpFeature::InputSource => "Input Source",
            VcpFeature::Brightness => "Brightness",
            VcpFeature::Contrast => "Contrast",
            VcpFeature::Volume => "Volume",
            VcpFeature::PowerMode => "Power Mode",
            VcpFeature::Custom(c) => mccs_name(*c).unwrap_or("Unknown"),
        }
    }
}

/// Standard MCCS 2.2 VCP code names for codes not directly modeled as
/// CLI-targetable variants. Returns `None` for unrecognized / manufacturer-
/// reserved codes the spec does not name.
fn mccs_name(code: u8) -> Option<&'static str> {
    Some(match code {
        0x02 => "New Control Value",
        0x04 => "Restore Factory Defaults",
        0x05 => "Restore Factory Luminance/Contrast Defaults",
        0x06 => "Restore Factory Geometry Defaults",
        0x08 => "Restore Factory Color Defaults",
        0x0A => "Restore Factory TV Defaults",
        0x0B => "Color Temperature Increment",
        0x0C => "Color Temperature Request",
        0x0E => "Clock",
        0x14 => "Select Color Preset",
        0x16 => "Video Gain: Red",
        0x17 => "User Color Vision Compensation",
        0x18 => "Video Gain: Green",
        0x1A => "Video Gain: Blue",
        0x1C => "Focus",
        0x1E => "Auto Setup",
        0x1F => "Auto Color Setup",
        0x20 => "Horizontal Position (Phase)",
        0x22 => "Horizontal Size",
        0x24 => "Horizontal Pincushion",
        0x26 => "Horizontal Pincushion Balance",
        0x28 => "Horizontal Convergence R/B",
        0x29 => "Horizontal Convergence M/G",
        0x2A => "Horizontal Linearity",
        0x2C => "Horizontal Linearity Balance",
        0x2E => "Gray Scale Expansion",
        0x30 => "Vertical Position (Phase)",
        0x32 => "Vertical Size",
        0x34 => "Vertical Pincushion",
        0x36 => "Vertical Pincushion Balance",
        0x38 => "Vertical Convergence R/B",
        0x39 => "Vertical Convergence M/G",
        0x3A => "Vertical Linearity",
        0x3C => "Vertical Linearity Balance",
        0x3E => "Clock Phase",
        0x40 => "Horizontal Parallelogram",
        0x41 => "Vertical Parallelogram",
        0x42 => "Horizontal Keystone",
        0x43 => "Vertical Keystone",
        0x44 => "Rotation",
        0x46 => "Top Corner Flare",
        0x48 => "Top Corner Hook",
        0x4A => "Bottom Corner Flare",
        0x4C => "Bottom Corner Hook",
        0x52 => "Active Control",
        0x54 => "Performance Preservation",
        0x56 => "Horizontal Moiré",
        0x58 => "Vertical Moiré",
        0x59 => "6 Axis Saturation: Red",
        0x5A => "6 Axis Saturation: Yellow",
        0x5B => "6 Axis Saturation: Green",
        0x5C => "6 Axis Saturation: Cyan",
        0x5D => "6 Axis Saturation: Blue",
        0x5E => "6 Axis Saturation: Magenta",
        0x66 => "Ambient Light Sensor",
        0x6B => "Backlight Level: White",
        0x6C => "Video Black Level: Red",
        0x6D => "Backlight Level: Red",
        0x6E => "Video Black Level: Green",
        0x6F => "Backlight Level: Green",
        0x70 => "Video Black Level: Blue",
        0x71 => "Backlight Level: Blue",
        0x72 => "Gamma",
        0x73 => "LUT Size",
        0x74 => "Single Point LUT Operation",
        0x75 => "Block LUT Operation",
        0x76 => "Remote Procedure Call",
        0x78 => "Display Identification Data Operation",
        0x7A => "Adjust Focal Plane",
        0x7C => "Adjust Zoom",
        0x82 => "Horizontal Mirror (Flip)",
        0x84 => "Vertical Mirror (Flip)",
        0x86 => "Display Scaling",
        0x87 => "Sharpness",
        0x88 => "Velocity Scan Modulation",
        0x8A => "Color Saturation",
        0x8B => "TV Channel Up/Down",
        0x8C => "TV Sharpness",
        0x8D => "Audio Mute / Screen Blank",
        0x8E => "TV Contrast",
        0x8F => "Audio Treble",
        0x90 => "Hue",
        0x91 => "Audio Bass",
        0x92 => "TV Black Level / Luminesce",
        0x93 => "Audio Balance L/R",
        0x94 => "Audio Processor Mode",
        0x95 => "Window Position (TL_X)",
        0x96 => "Window Position (TL_Y)",
        0x97 => "Window Position (BR_X)",
        0x98 => "Window Position (BR_Y)",
        0x99 => "Window Control On/Off",
        0x9A => "Window Background",
        0x9B => "6 Axis Hue Control: Red",
        0x9C => "6 Axis Hue Control: Yellow",
        0x9D => "6 Axis Hue Control: Green",
        0x9E => "6 Axis Hue Control: Cyan",
        0x9F => "6 Axis Hue Control: Blue",
        0xA0 => "6 Axis Hue Control: Magenta",
        0xA2 => "Auto Setup On/Off",
        0xA4 => "Window Mask Control",
        0xA5 => "Window Select",
        0xAA => "Screen Orientation",
        0xAC => "Horizontal Frequency",
        0xAE => "Vertical Frequency",
        0xB0 => "Settings",
        0xB2 => "Flat Panel Sub-pixel Layout",
        0xB4 => "Source Timing Mode",
        0xB6 => "Display Technology Type",
        0xB7 => "Monitor Status",
        0xB8 => "Packet Count",
        0xB9 => "Monitor X Origin",
        0xBA => "Monitor Y Origin",
        0xBB => "Header Error Count",
        0xBC => "Body CRC Error Count",
        0xBD => "Client ID",
        0xBE => "Link Control",
        0xC0 => "Display Usage Time",
        0xC2 => "Display Descriptor Length",
        0xC3 => "Transmit Display Descriptor",
        0xC4 => "Enable Display of Display Descriptor",
        0xC6 => "Application Enable Key",
        0xC8 => "Display Controller Type",
        0xC9 => "Display Firmware Level",
        0xCA => "OSD / Button Control",
        0xCC => "OSD Language",
        0xCD => "Status Indicators",
        0xCE => "Auxiliary Display Size",
        0xCF => "Auxiliary Display Data",
        0xD0 => "Output Select",
        0xD2 => "Asset Tag",
        0xD4 => "Stereo Video Mode",
        0xDA => "Scan Mode",
        0xDB => "Image Mode",
        0xDC => "Display Application",
        0xDF => "VCP Version",
        0xE0..=0xFF => "Manufacturer Specific",
        _ => return None,
    })
}

impl fmt::Display for VcpFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (0x{:02X})", self.name(), self.code())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputSource {
    Analog1,
    Analog2,
    Digital1,
    Digital2,
    Composite1,
    Composite2,
    SVideo1,
    SVideo2,
    Tuner1,
    Tuner2,
    Tuner3,
    Component1,
    Component2,
    Component3,
    DisplayPort1,
    DisplayPort2,
    Hdmi1,
    Hdmi2,
    UsbC,
    Unrecognized(u16),
}

impl InputSource {
    pub fn from_value(value: u16) -> Self {
        match value {
            0x01 => InputSource::Analog1,
            0x02 => InputSource::Analog2,
            0x03 => InputSource::Digital1, // Often DVI
            0x04 => InputSource::Digital2,
            0x05 => InputSource::Composite1,
            0x06 => InputSource::Composite2,
            0x07 => InputSource::SVideo1,
            0x08 => InputSource::SVideo2,
            0x09 => InputSource::Tuner1,
            0x0A => InputSource::Tuner2,
            0x0B => InputSource::Tuner3,
            0x0C => InputSource::Component1,
            0x0D => InputSource::Component2,
            0x0E => InputSource::Component3,
            0x0F => InputSource::DisplayPort1,
            0x10 => InputSource::DisplayPort2,
            0x11 => InputSource::Hdmi1,
            0x12 => InputSource::Hdmi2,
            // Note: USB-C often maps to DP or HDMI codes depending on the monitor
            v => InputSource::Unrecognized(v),
        }
    }

    pub fn value(&self) -> u16 {
        match self {
            InputSource::Analog1 => 0x01,
            InputSource::Analog2 => 0x02,
            InputSource::Digital1 => 0x03,
            InputSource::Digital2 => 0x04,
            InputSource::Composite1 => 0x05,
            InputSource::Composite2 => 0x06,
            InputSource::SVideo1 => 0x07,
            InputSource::SVideo2 => 0x08,
            InputSource::Tuner1 => 0x09,
            InputSource::Tuner2 => 0x0A,
            InputSource::Tuner3 => 0x0B,
            InputSource::Component1 => 0x0C,
            InputSource::Component2 => 0x0D,
            InputSource::Component3 => 0x0E,
            InputSource::DisplayPort1 => 0x0F,
            InputSource::DisplayPort2 => 0x10,
            InputSource::Hdmi1 => 0x11,
            InputSource::Hdmi2 => 0x12,
            InputSource::UsbC => 0x13, // Placeholder, often varies
            InputSource::Unrecognized(v) => *v,
        }
    }
}

impl fmt::Display for InputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputSource::Unrecognized(v) => write!(f, "Unknown(0x{:02X})", v),
            _ => write!(f, "{:?}", self),
        }
    }
}
