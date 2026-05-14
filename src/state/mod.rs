pub mod sled_db;

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fragment {
    pub text: String,
    pub group_id: Option<String>,
}

impl AsRef<str> for Fragment {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FragmentGroup {
    pub id: String,
    pub name: Option<String>,
}

pub fn log_tried_combination(combination: &[String], state_file: &Path) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(state_file)?;
    
    let line = combination.join(",") + "\n";
    file.write_all(line.as_bytes())?;
    Ok(())
}
