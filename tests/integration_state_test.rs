use encfs_cracker::fragment_combination::parallel::parallel_combination_test;
use encfs_cracker::state::sled_db::SledDb;
use tempfile::tempdir;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_parallel_combination_skips_tried() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("integration_test_db");
    let db = SledDb::init(&db_path).expect("Failed to init DB");
    
    // Clear trees for a fresh test
    db.tried_tree().unwrap().clear().unwrap();
    db.progress_tree().unwrap().clear().unwrap();
    db.save_checkpoint("0").unwrap();

    let fragments = vec!["a", "b", "c"];
    let k = 2;
    
    // Mark "a", "b" as tried
    db.mark_as_tried(&["a", "b"]).unwrap();
    
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = Arc::clone(&call_count);
    
    let validator = move |c: &[&str]| {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        // If we see "a", "b", something is wrong
        if c == &["a", "b"] {
            panic!("Validator should NOT be called for tried combination ['a', 'b']");
        }
        false
    };
    
    parallel_combination_test(&fragments, k, validator);
    
    // Total combinations for 3P2 = 3 * 2 = 6
    // (a,b), (a,c), (b,a), (b,c), (c,a), (c,b)
    // One skipped, so call_count should be 5
    assert_eq!(call_count.load(Ordering::SeqCst), 5, "Should have skipped one combination");
}

#[test]
fn test_parallel_combination_resume_from_checkpoint() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("resume_test_db");
    let db = SledDb::init(&db_path).expect("Failed to init DB");
    
    // Clear trees for a fresh test
    db.tried_tree().unwrap().clear().unwrap();
    db.progress_tree().unwrap().clear().unwrap();
    db.save_checkpoint("0").unwrap();

    let fragments = vec!["a", "b", "c"];
    let k = 2;
    
    // Save checkpoint: skip first 3 combinations
    // (a,b), (a,c), (b,a) -> should skip these
    db.save_checkpoint("3").unwrap();
    
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = Arc::clone(&call_count);
    
    let validator = move |c: &[&str]| {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        // We should only see (b,c), (c,a), (c,b)
        // Check if we see any of the first 3
        if c == &["a", "b"] || c == &["a", "c"] || c == &["b", "a"] {
            panic!("Validator should NOT be called for checkpointed combinations {:?}", c);
        }
        false
    };
    
    // Red Phase: Resume logic not implemented yet
    parallel_combination_test(&fragments, k, validator);
    
    // If resume works, only 3 should be tried
    assert_eq!(call_count.load(Ordering::SeqCst), 3, "Should have resumed from checkpoint 3");
}
