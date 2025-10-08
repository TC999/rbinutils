use crate::magic::{database::load_magic_rules, database::MagicRule};
use std::fs::File;
use std::io::Read;

pub fn detect_file_type(filename: &str) -> Result<String, String> {
    let mut file = File::open(filename).map_err(|e| e.to_string())?;
    let mut buffer = [0u8; 64];
    let n = file.read(&mut buffer).map_err(|e| e.to_string())?;

    for rule in load_magic_rules() {
        if n > rule.offset + rule.bytes.len() &&
           buffer[rule.offset..rule.offset + rule.bytes.len()] == rule.bytes[..]
        {
            return Ok(rule.description.clone());
        }
    }
    Ok("Unknown file type".to_string())
}