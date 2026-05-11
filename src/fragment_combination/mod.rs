use std::rc::Rc;

pub mod parallel;

pub fn generate_combinations<T: Clone + 'static>(fragments: &[T], k: usize) -> impl Iterator<Item = Vec<T>> {
    let fragments = Rc::new(fragments.to_vec());
    let n = fragments.len();
    
    if k == 0 {
        return Box::new(std::iter::once(vec![])) as Box<dyn Iterator<Item = Vec<T>>>;
    }
    if k > n {
        return Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Vec<T>>>;
    }
    
    Box::new(
        (0..n).flat_map(move |i| {
            let mut rest = (*fragments).clone();
            rest.remove(i);
            let fragments = Rc::clone(&fragments);
            
            if k == 1 {
                Box::new(std::iter::once(vec![fragments[i].clone()])) as Box<dyn Iterator<Item = Vec<T>>>
            } else {
                Box::new(generate_combinations(&rest, k - 1).map(move |c| {
                    let mut res = vec![fragments[i].clone()];
                    res.extend(c);
                    res
                }))
            }
        })
    )
}
