use crate::config::CrackerConfig;
use crate::fragment_combination::generate_combinations;
use crate::state::sled_db::SledDb;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ParallelCracker {
    config: Arc<CrackerConfig>,
    db: Arc<SledDb>,
    buffer: Arc<Mutex<Vec<Vec<String>>>>,
    is_running: Arc<AtomicBool>,
}

impl ParallelCracker {
    pub fn new(config: CrackerConfig) -> anyhow::Result<Self> {
        let db = SledDb::open(&config.db_path)?;
        let is_running = Arc::new(AtomicBool::new(true));
        
        let r = is_running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })?;

        Ok(Self { 
            config: Arc::new(config), 
            db: Arc::new(db),
            buffer: Arc::new(Mutex::new(Vec::new())),
            is_running,
        })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        for k in 1..=self.config.fragments.len() {
            let combinations: Vec<Vec<String>> = generate_combinations(&self.config.fragments, k).collect();
            
            let found = combinations.par_iter().try_for_each(|combo| -> anyhow::Result<()> {
                if !self.is_running.load(Ordering::SeqCst) {
                    return Ok(());
                }

                let combo_slice: Vec<&str> = combo.iter().map(|s| s.as_str()).collect();
                if self.db.is_tried(&combo_slice)? {
                    return Ok(());
                }
                
                let joined = combo.join("");
                if self.config.encfs_config.verify_password(&joined) {
                    println!("Password found: {}", joined);
                    std::fs::write("recovered_password.txt", &joined)?;
                    std::process::exit(0);
                }
                
                let mut buffer = self.buffer.lock().unwrap();
                buffer.push(combo.clone());
                
                if buffer.len() >= 1000 {
                    let batch: Vec<Vec<String>> = buffer.drain(..).collect();
                    drop(buffer);
                    for c in batch {
                        let c_slice: Vec<&str> = c.iter().map(|s| s.as_str()).collect();
                        self.db.mark_as_tried(&c_slice)?;
                    }
                }
                Ok(())
            });

            found?;
            
            if !self.is_running.load(Ordering::SeqCst) {
                break;
            }
            
            // Flush remaining buffer
            let mut buffer = self.buffer.lock().unwrap();
            let batch: Vec<Vec<String>> = buffer.drain(..).collect();
            drop(buffer);
            for c in batch {
                let c_slice: Vec<&str> = c.iter().map(|s| s.as_str()).collect();
                self.db.mark_as_tried(&c_slice)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::encfs_config::EncfSConfig;
    use tempfile::tempdir;

    #[test]
    fn test_parallel_cracker_runs() {
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
        let cracker = ParallelCracker::new(config).unwrap();
        cracker.run().unwrap();
    }
}
