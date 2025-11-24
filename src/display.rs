use crate::ddc;
use crate::error::DisplayError;
use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW};
use windows::Win32::Devices::Display::{GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, DestroyPhysicalMonitors, PHYSICAL_MONITOR};
use windows::Win32::Foundation::{LPARAM, HANDLE, RECT};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::fmt;
use windows::core::BOOL;
use serde::Serialize;

pub struct Display {
    pub handle: HANDLE,
    pub physical_monitor: PHYSICAL_MONITOR,
    pub name: String,
    pub id: usize,
}

impl Serialize for Display {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Display", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Display")
            .field("handle", &self.handle)
            .field("name", &self.name)
            .field("id", &self.id)
            .finish()
    }
}

impl Display {
    pub fn new(physical_monitor: PHYSICAL_MONITOR, name: String, id: usize) -> Self {
        Self {
            handle: physical_monitor.hPhysicalMonitor,
            physical_monitor,
            name,
            id,
        }
    }

    pub fn get_vcp_feature(&self, code: u8) -> Result<u32, DisplayError> {
        ddc::get_vcp_feature(self.handle, code)
    }

    pub fn set_vcp_feature(&self, code: u8, value: u32) -> Result<(), DisplayError> {
        ddc::set_vcp_feature(self.handle, code, value)
    }

    pub fn capabilities(&self) -> Result<String, DisplayError> {
        ddc::get_capabilities(self.handle)
    }
}

impl Drop for Display {
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
    // unsafe block for dereferencing lparam
    let displays = unsafe { &mut *(lparam.0 as *mut Vec<Display>) };
    
    // Get monitor info for the name
    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    let mut name = String::from("Unknown");

    // unsafe block for GetMonitorInfoW
    if unsafe { GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _).0 != 0 } {
        name = OsString::from_wide(&info.szDevice)
            .to_string_lossy()
            .trim_matches(char::from(0))
            .to_string();
    }

    // Get physical monitors
    let mut num_physical_monitors: u32 = 0;
    // unsafe block for GetNumberOfPhysicalMonitorsFromHMONITOR
    if unsafe { GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut num_physical_monitors).is_ok() } && num_physical_monitors > 0 {
        let mut physical_monitors = vec![PHYSICAL_MONITOR::default(); num_physical_monitors as usize];
        
        // unsafe block for GetPhysicalMonitorsFromHMONITOR
        if unsafe { GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut physical_monitors).is_ok() } {
            for pm in physical_monitors {
                let id = displays.len();
                // Fix unaligned access by copying the array
                let desc_array = pm.szPhysicalMonitorDescription;
                let pm_desc = OsString::from_wide(&desc_array)
                    .to_string_lossy()
                    .trim_matches(char::from(0))
                    .to_string();
                
                let display_name = if !pm_desc.is_empty() { pm_desc } else { name.clone() };
                
                displays.push(Display::new(pm, display_name, id));
            }
        }
    }

    BOOL(1)
}

pub fn enumerate_displays() -> Result<Vec<Display>, DisplayError> {
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
        return Err(DisplayError::MonitorNotFound("No monitors detected".to_string()));
    }

    Ok(displays)
}
