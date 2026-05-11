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
    
    assert_eq!(db.db.size_on_disk().unwrap(), same_db.db.size_on_disk().unwrap());
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
