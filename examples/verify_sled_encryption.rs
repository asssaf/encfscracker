use encfs_cracker::state::sled_db::SledDb;
use encfs_cracker::state::Fragment;
use tempfile::tempdir;

fn main() -> anyhow::Result<()> {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("manual_verify_db");
    let password = "manual_password";

    println!("--- Phase 2: Sled Integration Verification ---");

    // 1. Initialize DB with encryption
    println!("Initializing DB with encryption...");
    let db = SledDb::open(&db_path)?;
    db.initialize_encryption(password)?;

    // 2. Add some sensitive data
    println!("Adding sensitive fragment...");
    db.add_fragment(&Fragment {
        text: "sensitive_data_123".to_string(),
        group_id: None,
    })?;

    // 3. Close and Reopen
    println!("Closing and reopening DB...");
    drop(db);

    let db = SledDb::open(&db_path)?;

    // 4. Try access without unlock
    println!("Attempting access without unlock (expecting error)...");
    let res = db.list_fragments();
    if let Err(e) = res {
        println!("Access denied as expected: {}", e);
    } else {
        panic!("Access should have been denied!");
    }

    // 5. Unlock with correct password
    println!("Unlocking with correct password...");
    db.unlock(password)?;

    // 6. Verify data
    println!("Verifying data...");
    let fragments = db.list_fragments()?;
    assert_eq!(fragments[0].text, "sensitive_data_123");
    println!("Data verified: {}", fragments[0].text);

    println!("Verification SUCCESS: Sled integration is robust.");
    Ok(())
}
