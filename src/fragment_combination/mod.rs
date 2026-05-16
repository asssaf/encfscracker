use crate::state::Fragment;
use std::collections::HashSet;
use std::sync::Arc;

pub mod parallel;

pub trait Groupable {
    fn group_id(&self) -> Option<&str>;
}

impl Groupable for Fragment {
    fn group_id(&self) -> Option<&str> {
        self.group_id.as_deref()
    }
}

impl Groupable for String {
    fn group_id(&self) -> Option<&str> {
        None
    }
}

impl Groupable for &str {
    fn group_id(&self) -> Option<&str> {
        None
    }
}

pub fn generate_combinations<T: Groupable + Clone + Send + Sync + 'static>(
    fragments: &[T],
    k: usize,
) -> Box<dyn Iterator<Item = Vec<T>> + Send> {
    generate_combinations_recursive(fragments, k, HashSet::new())
}

fn generate_combinations_recursive<T: Groupable + Clone + Send + Sync + 'static>(
    fragments: &[T],
    k: usize,
    used_groups: HashSet<String>,
) -> Box<dyn Iterator<Item = Vec<T>> + Send> {
    let fragments = Arc::new(fragments.to_vec());
    let n = fragments.len();

    if k == 0 {
        return Box::new(std::iter::once(vec![]));
    }
    if k > n {
        return Box::new(std::iter::empty());
    }

    Box::new((0..n).flat_map(move |i| {
        let item = &fragments[i];

        // If item has a group and it's already used, skip this branch
        if let Some(gid) = item.group_id() {
            if used_groups.contains(gid) {
                return Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Vec<T>> + Send>;
            }
        }

        let mut new_used_groups = used_groups.clone();
        if let Some(gid) = item.group_id() {
            new_used_groups.insert(gid.to_string());
        }

        let mut rest = (*fragments).clone();
        rest.remove(i);
        let fragments = Arc::clone(&fragments);

        if k == 1 {
            Box::new(std::iter::once(vec![fragments[i].clone()]))
                as Box<dyn Iterator<Item = Vec<T>> + Send>
        } else {
            Box::new(
                generate_combinations_recursive(&rest, k - 1, new_used_groups).map(move |c| {
                    let mut res = vec![fragments[i].clone()];
                    res.extend(c);
                    res
                }),
            ) as Box<dyn Iterator<Item = Vec<T>> + Send>
        }
    }))
}
