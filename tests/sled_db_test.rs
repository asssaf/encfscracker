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
