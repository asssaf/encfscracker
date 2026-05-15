use encfs_cracker::state::sled_db::SledDb;
use tempfile::tempdir;

#[test]
fn test_sled_db_initialization() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test_db");
    
    let _db = SledDb::open(&db_path).expect("Failed to open DB");
    assert!(db_path.exists());
}

#[test]
fn test_sled_db_singleton() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("singleton_db");
    
    let db = SledDb::init(&db_path).expect("Failed to init DB");
    let same_db = SledDb::get().expect("Failed to get DB");
    
    assert!(std::ptr::eq(db, same_db));
}

#[test]
fn test_sled_db_trees() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("tree_test_db");
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    
    let tried_tree = db.tried_tree().unwrap();
    let progress_tree = db.progress_tree().unwrap();
    
    tried_tree.insert("test_pass", &[]).unwrap();
    progress_tree.insert(encfs_cracker::state::sled_db::KEY_CURRENT_CHECKPOINT, "123").unwrap();
    
    assert!(tried_tree.contains_key("test_pass").unwrap());
    assert_eq!(progress_tree.get(encfs_cracker::state::sled_db::KEY_CURRENT_CHECKPOINT).unwrap().unwrap(), "123".as_bytes());
}

#[test]
fn test_mark_and_check_tried_combinations() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("tried_test_db");
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    
    let combo = vec!["pass1", "pass2"];
    
    // Red Phase: These methods don't exist yet
    assert!(!db.is_tried(&combo).unwrap());
    db.mark_as_tried(&combo).unwrap();
    assert!(db.is_tried(&combo).unwrap());
}

#[test]
fn test_save_and_load_checkpoint() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("checkpoint_test_db");
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    db.initialize_encryption("test").unwrap();
    
    let checkpoint_val = "batch_1000";
    
    db.save_checkpoint(checkpoint_val).unwrap();
    let loaded = db.load_checkpoint().unwrap().expect("Checkpoint should exist");
    assert_eq!(loaded, checkpoint_val);
}

#[test]
fn test_reset_state() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("reset_test_db");
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    db.initialize_encryption("test").unwrap();
    
    db.mark_as_tried(&["a"]).unwrap();
    db.save_checkpoint("10").unwrap();
    
    db.reset_state().unwrap();
    
    assert!(!db.is_tried(&["a"]).unwrap());
    assert!(db.load_checkpoint().unwrap().is_none());
}
