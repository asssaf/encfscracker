use encfs_cracker::fragment_combination::parallel::parallel_combination_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_combination() {
        let fragments = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()];
        let k = 2;
        let validator = |c: &[String]| c == &[ "2".to_string(), "4".to_string() ];
        
        let found = parallel_combination_test(&fragments, k, validator, None);
        assert!(found);
    }
}
