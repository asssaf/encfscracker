use encfs_cracker::state::sled_db::SledDb;
use std::fs;

fn main() -> anyhow::Result<()> {
    let db_path = "sled_persistence_data";
    
    // Stage 1: Write data
    {
        let db = SledDb::open(db_path)?;
        let combo = vec!["test1", "test2"];
        let checkpoint = "phase2_done";
        
        db.mark_as_tried(&combo)?;
        db.save_checkpoint(checkpoint)?;
        println!("Stage 1: Saved combination and checkpoint.");
    }
    
    // Stage 2: Read data (new connection)
    {
        let db = SledDb::open(db_path)?;
        let combo = vec!["test1", "test2"];
        
        if db.is_tried(&combo)? {
            println!("Stage 2: Verified combination is persistent.");
        } else {
            anyhow::bail!("Stage 2: Combination NOT persistent!");
        }
        
        let loaded = db.load_checkpoint()?.expect("Checkpoint should exist");
        if loaded == "phase2_done" {
            println!("Stage 2: Verified checkpoint is persistent.");
        } else {
            anyhow::bail!("Stage 2: Checkpoint NOT persistent!");
        }
    }
    
    // Clean up
    fs::remove_dir_all(db_path)?;
    println!("Manual verification successful.");
    
    Ok(())
}
