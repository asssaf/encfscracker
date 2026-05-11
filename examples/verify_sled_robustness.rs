use encfs_cracker::state::sled_db::SledDb;
use std::fs;

fn main() -> anyhow::Result<()> {
    let db_path = "sled_robustness_data";
    
    // 1. Setup some state
    {
        let db = SledDb::init(db_path)?;
        db.mark_as_tried(&["robust", "test"])?;
        db.save_checkpoint("final")?;
        println!("State prepared.");
    }
    
    // 2. Reset state
    {
        let db = SledDb::get().expect("DB should be initialized");
        db.reset_state()?;
        println!("State reset called.");
        
        if db.is_tried(&["robust", "test"])? {
            anyhow::bail!("Verification failed: Tried combinations NOT cleared!");
        }
        if db.load_checkpoint()?.is_some() {
            anyhow::bail!("Verification failed: Checkpoint NOT cleared!");
        }
        println!("Verification successful: State is clean.");
    }
    
    // Clean up
    fs::remove_dir_all(db_path)?;
    
    Ok(())
}
