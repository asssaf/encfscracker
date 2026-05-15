use encfs_cracker::fragment_combination::generate_combinations;
use encfs_cracker::state::Fragment;
use std::time::Instant;

fn main() {
    let n = 12;
    let k = 4;
    
    // 1. Without groups
    let mut fragments_no_groups = Vec::new();
    for i in 0..n {
        fragments_no_groups.push(Fragment { text: i.to_string(), group_id: None });
    }
    
    let start = Instant::now();
    let count = generate_combinations(&fragments_no_groups, k).count();
    let duration = start.elapsed();
    println!("Without groups: {} combinations in {:?}", count, duration);
    
    // 2. With groups (all in different groups)
    let mut fragments_many_groups = Vec::new();
    for i in 0..n {
        fragments_many_groups.push(Fragment { text: i.to_string(), group_id: Some(i.to_string()) });
    }
    
    let start = Instant::now();
    let count = generate_combinations(&fragments_many_groups, k).count();
    let duration = start.elapsed();
    println!("With many groups: {} combinations in {:?}", count, duration);

    // 3. With groups (all in the same group - should be fast as it skips)
    let mut fragments_one_group = Vec::new();
    for i in 0..n {
        fragments_one_group.push(Fragment { text: i.to_string(), group_id: Some("G1".to_string()) });
    }
    
    let start = Instant::now();
    let count = generate_combinations(&fragments_one_group, k).count();
    let duration = start.elapsed();
    println!("With one group: {} combinations in {:?}", count, duration);
}
