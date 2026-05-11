use encfs_cracker::fragment_combination::parallel::parallel_combination_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_combination() {
        let fragments = vec![1, 2, 3, 4, 5];
        let k = 2;
        let validator = |c: &[i32]| c == &[2, 4];
        
        let found = parallel_combination_test(&fragments, k, validator);
        assert!(found);
    }
}
