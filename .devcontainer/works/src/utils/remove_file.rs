use crate::constants::constants::DIR;
use std::fs;
use std::path::Path;

pub fn remove_file_if_exists(file_name: &str) -> std::io::Result<()> {
    let dir = Path::new(DIR);
    let path = dir.join(file_name);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
