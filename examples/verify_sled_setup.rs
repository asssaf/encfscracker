use encfs_cracker::state::sled_db::SledDb;
use std::fs;

fn main() -> anyhow::Result<()> {
    let db_path = "sled_data";

    // Clean up previous run if any
    if std::path::Path::new(db_path).exists() {
        fs::remove_dir_all(db_path)?;
    }

    let db = SledDb::open(db_path)?;
    println!("Database initialized successfully.");

    let _tried = db.tried_tree()?;
    let _progress = db.progress_tree()?;
    println!("Trees 'tried_combinations' and 'progress' are accessible.");

    if std::path::Path::new(db_path).exists() {
        println!("Verified: 'sled_data' directory exists.");
    }

    // Clean up
    fs::remove_dir_all(db_path)?;

    Ok(())
}
