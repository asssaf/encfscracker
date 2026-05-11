pub mod sled_db;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn log_tried_combination(combination: &[String], state_file: &Path) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(state_file)?;
    
    let line = combination.join(",") + "\n";
    file.write_all(line.as_bytes())?;
    Ok(())
}
