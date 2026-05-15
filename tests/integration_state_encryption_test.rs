use encfs_cracker::state::sled_db::SledDb;
use encfs_cracker::state::Fragment;
use tempfile::tempdir;

#[test]
fn test_sled_db_encryption_flow() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("encrypted_db");
    let password = "super_secret_password";
    
    // 1. Open new DB and check initialization status
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    assert!(db.needs_initialization().unwrap());
    
    // 2. Initialize encryption
    db.initialize_encryption(password).expect("Failed to initialize encryption");
    assert!(!db.needs_initialization().unwrap());
    
    // 3. Add data
    let fragment = Fragment {
        text: "secret_fragment".to_string(),
        group_id: None,
    };
    db.add_fragment(&fragment).expect("Failed to add fragment");
    
    db.save_checkpoint("step_100").expect("Failed to save checkpoint");
    db.mark_as_tried(&["pass1", "pass2"]).expect("Failed to mark as tried");
    
    // 4. Verify data in same session
    let fragments = db.list_fragments().expect("Failed to list fragments");
    assert_eq!(fragments.len(), 1);
    assert_eq!(fragments[0].text, "secret_fragment");
    
    let checkpoint = db.load_checkpoint().expect("Failed to load checkpoint");
    assert_eq!(checkpoint.unwrap(), "step_100");
    
    assert!(db.is_tried(&["pass1", "pass2"]).expect("Failed to check if tried"));
}

#[test]
fn test_sled_db_unlock_flow() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("unlock_db");
    let password = "correct_password";
    let wrong_password = "wrong_password";
    
    {
        let db = SledDb::open(&db_path).expect("Failed to open DB");
        db.initialize_encryption(password).expect("Failed to initialize");
        db.save_checkpoint("done").expect("Failed to save");
    } // DB closed
    
    // 1. Try to unlock with wrong password
    {
        let db = SledDb::open(&db_path).expect("Failed to reopen DB");
        assert!(!db.needs_initialization().unwrap());
        let result = db.unlock(wrong_password);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Incorrect password"));
        
        // Data access should fail when locked
        assert!(db.load_checkpoint().is_err());
    }
    
    // 2. Unlock with correct password
    {
        let db = SledDb::open(&db_path).expect("Failed to reopen DB again");
        db.unlock(password).expect("Failed to unlock");
        let checkpoint = db.load_checkpoint().expect("Failed to load");
        assert_eq!(checkpoint.unwrap(), "done");
    }
}

#[test]
fn test_tried_combinations_privacy() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("privacy_db");
    let password = "password";
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    db.initialize_encryption(password).expect("Init failed");
    
    let combo = ["sensitive_pass"];
    db.mark_as_tried(&combo).expect("Mark failed");
    
    // Inspect raw tree to ensure plaintext combination is not there
    let tried_tree = db.tried_tree().unwrap();
    for item in tried_tree.iter() {
        let (k, _) = item.unwrap();
        let k_str = String::from_utf8_lossy(&k);
        assert!(!k_str.contains("sensitive_pass"), "Plaintext combination found in database!");
    }
}
