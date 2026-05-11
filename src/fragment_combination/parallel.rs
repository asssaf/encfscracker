use rayon::prelude::*;
use crate::fragment_combination::generate_combinations;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn parallel_combination_test<T, F>(fragments: &[T], k: usize, validator: F) -> bool 
where 
    T: Clone + Send + Sync + 'static,
    F: Fn(&[T]) -> bool + Send + Sync 
{
    let count = AtomicUsize::new(0);
    
    generate_combinations(fragments, k).par_bridge().any(|c| {
        let current = count.fetch_add(1, Ordering::SeqCst);
        if current % 100 == 0 {
            println!("Tried {} combinations...", current);
        }
        validator(&c)
    })
}
