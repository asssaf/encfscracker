use encfs_cracker::state::sled_db::SledDb;
use encfs_cracker::state::{Fragment, FragmentGroup};
use tempfile::tempdir;

#[test]
fn test_fragment_creation_and_persistence() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("fragment_test_db");
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    
    let fragment = Fragment {
        text: "secret".to_string(),
        group_id: Some("A".to_string()),
    };
    
    // These methods don't exist yet
    db.add_fragment(&fragment).expect("Failed to add fragment");
    
    let fragments = db.list_fragments().expect("Failed to list fragments");
    assert_eq!(fragments.len(), 1);
    assert_eq!(fragments[0].text, "secret");
    assert_eq!(fragments[0].group_id, Some("A".to_string()));
}

#[test]
fn test_group_persistence() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("group_test_db");
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    
    let group = FragmentGroup {
        id: "B".to_string(),
        name: Some("Group B".to_string()),
    };
    
    db.add_group(&group).expect("Failed to add group");
    
    let groups = db.list_groups().expect("Failed to list groups");
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].id, "B");
    assert_eq!(groups[0].name, Some("Group B".to_string()));
}

#[test]
fn test_clear_fragments_and_groups() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("clear_test_db");
    let db = SledDb::open(&db_path).expect("Failed to open DB");
    
    db.add_fragment(&Fragment {
        text: "f1".to_string(),
        group_id: None,
    }).unwrap();
    
    db.add_group(&FragmentGroup {
        id: "G1".to_string(),
        name: None,
    }).unwrap();
    
    assert_eq!(db.list_fragments().unwrap().len(), 1);
    assert_eq!(db.list_groups().unwrap().len(), 1);
    
    db.clear_fragments().unwrap();
    assert_eq!(db.list_fragments().unwrap().len(), 0);
    assert_eq!(db.list_groups().unwrap().len(), 1);
    
    db.clear_groups().unwrap();
    assert_eq!(db.list_groups().unwrap().len(), 0);
}
