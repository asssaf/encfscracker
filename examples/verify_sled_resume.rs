use encfs_cracker::fragment_combination::parallel::parallel_combination_test;
use encfs_cracker::state::sled_db::SledDb;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() -> anyhow::Result<()> {
    let db_path = "sled_resume_data";
    
    // Clean up
    if std::path::Path::new(db_path).exists() {
        fs::remove_dir_all(db_path)?;
    }
    
    let db = SledDb::init(db_path)?;
    let fragments = vec!["a", "b", "c", "d"];
    let k = 2; // 4P2 = 12 combinations
    
    println!("Step 1: Running first pass, will stop halfway.");
    let count = AtomicUsize::new(0);
    parallel_combination_test(&fragments, k, |c| {
        let current = count.fetch_add(1, Ordering::SeqCst);
        if current >= 5 {
            return true; // Stop early
        }
        println!("Pass 1: Tried {:?}", c);
        false
    });
    
    println!("Step 2: Resuming from checkpoint.");
    let count2 = AtomicUsize::new(0);
    parallel_combination_test(&fragments, k, |c| {
        count2.fetch_add(1, Ordering::SeqCst);
        println!("Pass 2: Tried {:?}", c);
        false
    });
    
    let total_tried_pass2 = count2.load(Ordering::SeqCst);
    println!("Step 2 tried {} combinations.", total_tried_pass2);
    
    if total_tried_pass2 < 12 {
        println!("Manual verification successful: Resume logic skipped already tried combinations.");
    } else {
        anyhow::bail!("Manual verification failed: Resume logic did NOT skip enough combinations.");
    }

    // Clean up
    fs::remove_dir_all(db_path)?;
    
    Ok(())
}
