use encfs_cracker::fragment_combination::parallel::parallel_combination_test;

fn main() {
    let fragments = vec!["a", "b", "c", "d"];
    let k = 2;
    let validator = |c: &[&str]| {
        println!("Checking combination: {:?}", c);
        c == &["c", "a"]
    };
    
    let found = parallel_combination_test(&fragments, k, validator, None);
    if found {
        println!("Success: Found combination!");
    } else {
        println!("Failure: Combination not found.");
    }
}
