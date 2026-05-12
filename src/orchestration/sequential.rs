use crate::config::CrackerConfig;
use crate::fragment_combination::generate_combinations;
use crate::state::sled_db::SledDb;

pub struct SequentialCracker {
    config: CrackerConfig,
    db: SledDb,
}

impl SequentialCracker {
    pub fn new(config: CrackerConfig) -> anyhow::Result<Self> {
        let db = SledDb::open(&config.db_path)?;
        Ok(Self { config, db })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        for k in 1..=self.config.fragments.len() {
            let combinations = generate_combinations(&self.config.fragments, k);
            for combo in combinations {
                let combo_slice: Vec<&str> = combo.iter().map(|s| s.as_str()).collect();
                if self.db.is_tried(&combo_slice)? {
                    continue;
                }
                
                let joined = combo.join("");
                if self.config.encfs_config.verify_password(&joined) {
                    println!("Password found: {}", joined);
                    std::fs::write("recovered_password.txt", &joined)?;
                    std::process::exit(0);
                }
                
                self.db.mark_as_tried(&combo_slice)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::encfs_config::EncfSConfig;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_sequential_cracker_finds_password() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        let xml = r#"<config>
    <salt>SGVsbG8=</salt>
    <iterations>1000</iterations>
    <keySize>32</keySize>
    <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
</config>"#;
        let encfs_config = EncfSConfig::from_xml(xml).unwrap();
        let config = CrackerConfig {
            fragments: vec!["a".to_string(), "b".to_string()],
            encfs_config,
            db_path,
        };
        let cracker = SequentialCracker::new(config).unwrap();
        cracker.run().unwrap();
    }
}
