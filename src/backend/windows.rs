use super::{DdcBackend, Display, DisplayInfo};
use crate::error::DisplayError;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Devices::Display::{
    CapabilitiesRequestAndCapabilitiesReply, DestroyPhysicalMonitors, GetCapabilitiesStringLength,
    GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR,
    GetVCPFeatureAndVCPFeatureReply, PHYSICAL_MONITOR, SetVCPFeature,
};
use windows::Win32::Foundation::{HANDLE, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW,
};
use windows::core::BOOL;

struct WindowsBackend {
    handle: HANDLE,
    physical_monitor: PHYSICAL_MONITOR,
}

impl DdcBackend for WindowsBackend {
    fn get_vcp(&mut self, code: u8) -> Result<u32, DisplayError> {
        let mut current_value: u32 = 0;
        let mut max_value: u32 = 0;
        let success = unsafe {
            GetVCPFeatureAndVCPFeatureReply(
                self.handle,
                code,
                None,
                &mut current_value,
                Some(&mut max_value),
            )
        };

        if success != 0 {
            Ok(current_value)
        } else {
            Err(DisplayError::DdcCommunicationFailed(format!(
                "GetVCPFeatureAndVCPFeatureReply failed for code 0x{:02X}",
                code
            )))
        }
    }

    fn set_vcp(&mut self, code: u8, value: u32) -> Result<(), DisplayError> {
        let success = unsafe { SetVCPFeature(self.handle, code, value) };

        if success != 0 {
            Ok(())
        } else {
            Err(DisplayError::DdcCommunicationFailed(format!(
                "SetVCPFeature failed for code 0x{:02X}",
                code
            )))
        }
    }

    fn capabilities(&mut self) -> Result<String, DisplayError> {
        let mut length: u32 = 0;
        let success = unsafe { GetCapabilitiesStringLength(self.handle, &mut length) };

        if success == 0 {
            return Err(DisplayError::DdcCommunicationFailed(
                "GetCapabilitiesStringLength failed".to_string(),
            ));
        }

        let mut buffer = vec![0u8; length as usize];
        let success_str =
            unsafe { CapabilitiesRequestAndCapabilitiesReply(self.handle, &mut buffer) };

        if success_str == 0 {
            return Err(DisplayError::DdcCommunicationFailed(
                "CapabilitiesRequestAndCapabilitiesReply failed".to_string(),
            ));
        }

        let s = String::from_utf8_lossy(&buffer).to_string();
        Ok(s.trim_matches(char::from(0)).to_string())
    }
}

impl Drop for WindowsBackend {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyPhysicalMonitors(&[self.physical_monitor]);
        }
    }
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let displays = unsafe { &mut *(lparam.0 as *mut Vec<Display>) };

    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    let mut device_name = String::from("Unknown");

    if unsafe { GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _).0 != 0 } {
        device_name = OsString::from_wide(&info.szDevice)
            .to_string_lossy()
            .trim_matches(char::from(0))
            .to_string();
    }

    let mut num_physical_monitors: u32 = 0;
    if unsafe { GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut num_physical_monitors).is_ok() }
        && num_physical_monitors > 0
    {
        let mut physical_monitors = vec![PHYSICAL_MONITOR::default(); num_physical_monitors as usize];

        if unsafe { GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut physical_monitors).is_ok() } {
            for pm in physical_monitors {
                let id = displays.len();
                let desc_array = pm.szPhysicalMonitorDescription;
                let pm_desc = OsString::from_wide(&desc_array)
                    .to_string_lossy()
                    .trim_matches(char::from(0))
                    .to_string();

                let name = if !pm_desc.is_empty() {
                    pm_desc
                } else {
                    device_name.clone()
                };

                // stable_id: fall back to Windows device path until EDID extraction lands.
                let stable_id = device_name.clone();

                let info = DisplayInfo { name, stable_id };
                let backend = WindowsBackend {
                    handle: pm.hPhysicalMonitor,
                    physical_monitor: pm,
                };

                displays.push(Display::new(id, info, Box::new(backend)));
            }
        }
    }

    BOOL(1)
}

pub fn enumerate() -> Result<Vec<Display>, DisplayError> {
    let mut displays: Vec<Display> = Vec::new();

    unsafe {
        let _ = EnumDisplayMonitors(
            None,
            None,
            Some(monitor_enum_proc),
            LPARAM(&mut displays as *mut _ as isize),
        );
    }

    if displays.is_empty() {
        return Err(DisplayError::MonitorNotFound(
            "No monitors detected".to_string(),
        ));
    }

    Ok(displays)
}
