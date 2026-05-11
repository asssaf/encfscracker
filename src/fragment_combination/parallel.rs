use rayon::prelude::*;
use crate::fragment_combination::generate_combinations;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::state::sled_db::SledDb;

pub fn parallel_combination_test<T, F>(fragments: &[T], k: usize, validator: F) -> bool 
where 
    T: Clone + Send + Sync + 'static + AsRef<str>,
    F: Fn(&[T]) -> bool + Send + Sync 
{
    let skip_count = if let Some(db) = SledDb::get() {
        db.load_checkpoint().unwrap_or(None).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0)
    } else {
        0
    };

    let count = AtomicUsize::new(skip_count);
    
    generate_combinations(fragments, k).skip(skip_count).par_bridge().any(|c| {
        if let Some(db) = SledDb::get() {
            let string_c: Vec<&str> = c.iter().map(|s| s.as_ref()).collect();
            if db.is_tried(&string_c).unwrap_or(false) {
                return false;
            }
        }

        let current = count.fetch_add(1, Ordering::SeqCst);
        if current % 100 == 0 {
            println!("Tried {} combinations...", current);
            if let Some(db) = SledDb::get() {
                let _ = db.save_checkpoint(&current.to_string());
            }
        }
        
        let result = validator(&c);
        
        if result {
            return true;
        }

        if let Some(db) = SledDb::get() {
            let string_c: Vec<&str> = c.iter().map(|s| s.as_ref()).collect();
            let _ = db.mark_as_tried(&string_c);
        }

        false
    })
}
