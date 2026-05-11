use rayon::prelude::*;
use crate::fragment_combination::generate_combinations;

pub fn parallel_combination_test<T, F>(fragments: &[T], k: usize, validator: F) -> bool 
where 
    T: Clone + Send + Sync + 'static,
    F: Fn(&[T]) -> bool + Send + Sync 
{
    let combinations: Vec<Vec<T>> = generate_combinations(fragments, k).collect();
    combinations.into_par_iter().any(|c| validator(&c))
}
