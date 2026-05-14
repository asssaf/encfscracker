use crate::config::CrackerConfig;
use crate::fragment_combination::generate_combinations;
use crate::state::sled_db::SledDb;
use crate::state::Fragment;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ParallelCracker {
    config: Arc<CrackerConfig>,
    db: Arc<SledDb>,
    buffer: Arc<Mutex<Vec<Vec<Fragment>>>>,
    is_running: Arc<AtomicBool>,
}

impl ParallelCracker {
    pub fn new(config: CrackerConfig) -> anyhow::Result<Self> {
        let db = SledDb::open(&config.db_path)?;
        let is_running = Arc::new(AtomicBool::new(true));
        
        #[cfg(not(test))]
        {
            let r = is_running.clone();
            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })?;
        }

        Ok(Self { 
            config: Arc::new(config), 
            db: Arc::new(db),
            buffer: Arc::new(Mutex::new(Vec::new())),
            is_running,
        })
    }

    pub fn run(&self) -> anyhow::Result<Option<String>> {
        let start_time = std::time::Instant::now();
        let total_attempts = Arc::new(std::sync::atomic::AtomicU64::new(0));
        
        for k in 1..=self.config.fragments.len() {
            println!("Testing combinations of length {}...", k);
            let combinations = generate_combinations(&self.config.fragments, k);
            
            let total_attempts_inner = total_attempts.clone();
            let is_running = self.is_running.clone();
            let found_password = Arc::new(Mutex::new(None));
            let found_password_inner = found_password.clone();

            combinations.par_bridge().try_for_each(|combo| -> anyhow::Result<()> {
                if !is_running.load(Ordering::SeqCst) || found_password_inner.lock().unwrap().is_some() {
                    return Ok(());
                }

                let combo_slice: Vec<&str> = combo.iter().map(|s| s.text.as_str()).collect();
                if self.db.is_tried(&combo_slice)? {
                    return Ok(());
                }
                
                let joined: String = combo.iter().map(|f| f.text.as_str()).collect();
                if self.config.encfs_config.verify_password(&joined) {
                    let mut found = found_password_inner.lock().unwrap();
                    *found = Some(joined);
                    return Ok(());
                }
                
                let current = total_attempts_inner.fetch_add(1, Ordering::SeqCst);
                if current > 0 && current % 10000 == 0 {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = current as f64 / elapsed;
                    println!("Tried {} combinations... ({:.2} combinations/sec)", current, speed);
                }

                let mut buffer = self.buffer.lock().unwrap();
                buffer.push(combo.clone());
                
                if buffer.len() >= 1000 {
                    let batch: Vec<Vec<Fragment>> = buffer.drain(..).collect();
                    drop(buffer);
                    for c in batch {
                        let c_slice: Vec<&str> = c.iter().map(|s| s.text.as_str()).collect();
                        self.db.mark_as_tried(&c_slice)?;
                    }
                }
                Ok(())
            })?;

            let found = found_password.lock().unwrap().take();
            if let Some(password) = found {
                return Ok(Some(password));
            }
            
            if !self.is_running.load(Ordering::SeqCst) {
                break;
            }
            
            // Flush remaining buffer
            let mut buffer = self.buffer.lock().unwrap();
            let batch: Vec<Vec<Fragment>> = buffer.drain(..).collect();
            drop(buffer);
            for c in batch {
                let c_slice: Vec<&str> = c.iter().map(|s| s.text.as_str()).collect();
                self.db.mark_as_tried(&c_slice)?;
            }
        }
        Ok(None)
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
        
        let xml = r#"<boost_serialization>
    <cfg>
        <saltData>SGVsbG8=</saltData>
        <kdfIterations>1000</kdfIterations>
        <keySize>32</keySize>
        <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
    </cfg>
</boost_serialization>"#;
        let encfs_config = EncfSConfig::from_xml(xml).unwrap();
        let config = CrackerConfig {
            fragments: vec![
                Fragment { text: "a".to_string(), group_id: None },
                Fragment { text: "b".to_string(), group_id: None }
            ],
            encfs_config,
            db_path,
        };
        let cracker = ParallelCracker::new(config).unwrap();
        let _ = cracker.run().unwrap();
    }
}
