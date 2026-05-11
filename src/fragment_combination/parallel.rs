use rayon::prelude::*;
use crate::fragment_combination::generate_combinations;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::state::sled_db::SledDb;

pub fn parallel_combination_test<T, F>(fragments: &[T], k: usize, validator: F) -> bool 
where 
    T: Clone + Send + Sync + 'static + AsRef<str>,
    F: Fn(&[T]) -> bool + Send + Sync 
{
    let count = AtomicUsize::new(0);
    
    generate_combinations(fragments, k).par_bridge().any(|c| {
        if let Some(db) = SledDb::get() {
            let string_c: Vec<&str> = c.iter().map(|s| s.as_ref()).collect();
            if db.is_tried(&string_c).unwrap_or(false) {
                return false;
            }
        }

        let current = count.fetch_add(1, Ordering::SeqCst);
        if current % 100 == 0 {
            println!("Tried {} combinations...", current);
        }
        
        let result = validator(&c);
        
        if result {
            return true;
        }

        // Optional: mark as tried here?
        // The plan says "skip combinations where is_tried returns true".
        // Usually we mark as tried AFTER we tried it.
        if let Some(db) = SledDb::get() {
            let string_c: Vec<&str> = c.iter().map(|s| s.as_ref()).collect();
            let _ = db.mark_as_tried(&string_c);
        }

        false
    })
}
