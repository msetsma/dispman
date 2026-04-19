use super::{DdcBackend, Display, DisplayInfo};
use crate::error::DisplayError;
use ddc::Ddc;
use ddc_macos::Monitor;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct MacOsBackend {
    monitor: Monitor,
}

impl DdcBackend for MacOsBackend {
    fn get_vcp(&mut self, code: u8) -> Result<u32, DisplayError> {
        self.monitor
            .get_vcp_feature(code)
            .map(|v| u32::from(v.value()))
            .map_err(|e| {
                DisplayError::DdcCommunicationFailed(format!(
                    "get_vcp_feature(0x{:02X}) failed: {}",
                    code, e
                ))
            })
    }

    fn set_vcp(&mut self, code: u8, value: u32) -> Result<(), DisplayError> {
        let narrowed: u16 = u16::try_from(value).map_err(|_| {
            DisplayError::FeatureNotSupported(format!(
                "Value {} does not fit in u16 for VCP 0x{:02X}",
                value, code
            ))
        })?;

        self.monitor.set_vcp_feature(code, narrowed).map_err(|e| {
            DisplayError::DdcCommunicationFailed(format!(
                "set_vcp_feature(0x{:02X}, {}) failed: {}",
                code, value, e
            ))
        })
    }

    fn capabilities(&mut self) -> Result<String, DisplayError> {
        let bytes = self.monitor.capabilities_string().map_err(|e| {
            DisplayError::DdcCommunicationFailed(format!("capabilities_string failed: {}", e))
        })?;
        Ok(String::from_utf8_lossy(&bytes)
            .trim_matches(char::from(0))
            .to_string())
    }
}

pub fn enumerate() -> Result<Vec<Display>, DisplayError> {
    let monitors = Monitor::enumerate().map_err(|e| {
        DisplayError::DdcCommunicationFailed(format!("Monitor::enumerate failed: {}", e))
    })?;

    if monitors.is_empty() {
        return Err(DisplayError::MonitorNotFound(
            "No external monitors detected".to_string(),
        ));
    }

    let mut displays = Vec::with_capacity(monitors.len());
    for (id, mut monitor) in monitors.into_iter().enumerate() {
        let name = monitor
            .product_name()
            .unwrap_or_else(|| monitor.description());
        let stable_id = derive_stable_id(&mut monitor, &name);
        let info = DisplayInfo { name, stable_id };
        let backend = MacOsBackend { monitor };
        displays.push(Display::new(id, info, Box::new(backend)));
    }

    Ok(displays)
}

fn derive_stable_id(monitor: &mut Monitor, fallback_name: &str) -> String {
    if let Some(serial) = monitor.serial_number() {
        if !serial.trim().is_empty() {
            return format!("serial:{}", serial);
        }
    }
    if let Some(edid) = monitor.edid() {
        let mut hasher = DefaultHasher::new();
        edid.hash(&mut hasher);
        return format!("edid:{:016x}", hasher.finish());
    }
    format!("name:{}", fallback_name)
}
