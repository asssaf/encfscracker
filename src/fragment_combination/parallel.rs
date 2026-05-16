use crate::fragment_combination::generate_combinations;
use crate::state::sled_db::SledDb;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn parallel_combination_test<T, F>(
    fragments: &[T],
    k: usize,
    validator: F,
    db: Option<&SledDb>,
) -> bool
where
    T: crate::fragment_combination::Groupable
        + Clone
        + Send
        + Sync
        + 'static
        + AsRef<str>
        + std::fmt::Debug,
    F: Fn(&[T]) -> bool + Send + Sync,
{
    let skip_count = if let Some(db) = db {
        db.load_checkpoint()
            .unwrap_or(None)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0)
    } else {
        0
    };

    let count = AtomicUsize::new(skip_count);

    let combinations: Vec<_> = generate_combinations(fragments, k)
        .skip(skip_count)
        .collect();

    combinations.par_iter().any(|c| {
        if let Some(db) = db {
            let string_c: Vec<&str> = c.iter().map(|s| s.as_ref()).collect();
            if db.is_tried(&string_c).unwrap_or(false) {
                return false;
            }
        }

        let _current = count.fetch_add(1, Ordering::SeqCst);

        let result = validator(c);

        if result {
            return true;
        }

        if let Some(db) = db {
            let string_c: Vec<&str> = c.iter().map(|s| s.as_ref()).collect();
            let _ = db.mark_as_tried(&string_c);
        }

        false
    })
}
