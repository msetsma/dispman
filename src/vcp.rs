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
            VcpFeature::Custom(_) => "Custom",
        }
    }
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
