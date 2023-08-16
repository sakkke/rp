use crate::rp::RpConfig;
use std::fs::File;
use std::io::Read;

pub fn read_config(path: &str) -> Result<RpConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    serde_json::from_str(&content).map_err(|e| e.into())
}
