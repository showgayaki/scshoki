use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct DisplaySpec {
    pub ppi: u16,
    pub width: u16,
    pub height: u16,
    pub scale_factor: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeviceSpec {
    pub model_name: String,
    pub display: DisplaySpec,
}

pub type DeviceSpecs = HashMap<String, DeviceSpec>;
