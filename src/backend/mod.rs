use crate::error::DisplayError;
use serde::Serialize;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Debug, Clone, Serialize)]
pub struct DisplayInfo {
    pub name: String,
    pub stable_id: String,
}

pub struct Display {
    pub id: usize,
    pub info: DisplayInfo,
    inner: Box<dyn DdcBackend>,
}

impl Display {
    pub fn new(id: usize, info: DisplayInfo, inner: Box<dyn DdcBackend>) -> Self {
        Self { id, info, inner }
    }

    pub fn name(&self) -> &str {
        &self.info.name
    }

    pub fn stable_id(&self) -> &str {
        &self.info.stable_id
    }

    pub fn get_vcp_feature(&mut self, code: u8) -> Result<u32, DisplayError> {
        self.inner.get_vcp(code)
    }

    pub fn set_vcp_feature(&mut self, code: u8, value: u32) -> Result<(), DisplayError> {
        self.inner.set_vcp(code, value)
    }

    pub fn capabilities(&mut self) -> Result<String, DisplayError> {
        self.inner.capabilities()
    }
}

impl Serialize for Display {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Display", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.info.name)?;
        state.serialize_field("stable_id", &self.info.stable_id)?;
        state.end()
    }
}

impl std::fmt::Debug for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Display")
            .field("id", &self.id)
            .field("info", &self.info)
            .finish()
    }
}

pub trait DdcBackend {
    fn get_vcp(&mut self, code: u8) -> Result<u32, DisplayError>;
    fn set_vcp(&mut self, code: u8, value: u32) -> Result<(), DisplayError>;
    fn capabilities(&mut self) -> Result<String, DisplayError>;
}

pub fn enumerate() -> Result<Vec<Display>, DisplayError> {
    #[cfg(target_os = "windows")]
    {
        windows::enumerate()
    }
    #[cfg(target_os = "macos")]
    {
        macos::enumerate()
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Err(DisplayError::UnsupportedPlatform)
    }
}
