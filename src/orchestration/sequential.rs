use crate::config::CrackerConfig;
use crate::fragment_combination::generate_combinations;
use crate::state::sled_db::SledDb;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SequentialCracker {
    config: CrackerConfig,
    db: SledDb,
    is_running: Arc<AtomicBool>,
}

impl SequentialCracker {
    pub fn new(config: CrackerConfig) -> anyhow::Result<Self> {
        let db = SledDb::open(&config.db_path)?;
        let is_running = Arc::new(AtomicBool::new(true));
        
        let r = is_running.clone();
        #[cfg(not(test))]
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })?;

        Ok(Self { config, db, is_running })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        for k in 1..=self.config.fragments.len() {
            let combinations = generate_combinations(&self.config.fragments, k);

            for combo in combinations {
                if !self.is_running.load(Ordering::SeqCst) {
                    return Ok(());
                }

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

mod tests {
    use super::*;
    use crate::crypto::encfs_config::EncfSConfig;
    use std::path::PathBuf;
    use ::tempfile::tempdir;

    #[test]
    fn test_sequential_cracker_finds_password() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        let xml = r#"<boost_serialization>
    <cfg>
        <saltData>SGVsbG8=</saltData>
        <kdfIterations>1000</kdfIterations>
        <keySize>32</keySize>
        <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
    </cfg>
</boost_serialization>"#;
        println!("XML: {}", xml);
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
