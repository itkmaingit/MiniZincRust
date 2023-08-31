use std::fs;
use std::path::Path;

pub fn remove_file_if_exists(output_txt_path: &str) -> std::io::Result<()> {
    let path = Path::new(output_txt_path);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
