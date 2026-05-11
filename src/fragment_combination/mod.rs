use std::sync::Arc;

pub mod parallel;

pub fn generate_combinations<T: Clone + Send + Sync + 'static>(fragments: &[T], k: usize) -> Box<dyn Iterator<Item = Vec<T>> + Send> {
    let fragments = Arc::new(fragments.to_vec());
    let n = fragments.len();
    
    if k == 0 {
        return Box::new(std::iter::once(vec![]));
    }
    if k > n {
        return Box::new(std::iter::empty());
    }
    
    Box::new(
        (0..n).flat_map(move |i| {
            let mut rest = (*fragments).clone();
            rest.remove(i);
            let fragments = Arc::clone(&fragments);
            
            if k == 1 {
                Box::new(std::iter::once(vec![fragments[i].clone()])) as Box<dyn Iterator<Item = Vec<T>> + Send>
            } else {
                Box::new(generate_combinations(&rest, k - 1).map(move |c| {
                    let mut res = vec![fragments[i].clone()];
                    res.extend(c);
                    res
                })) as Box<dyn Iterator<Item = Vec<T>> + Send>
            }
        })
    )
}
