use encfs_cracker::fragment_combination::generate_combinations;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_generation() {
        let fragments = vec!["a", "b", "c"];
        let combinations: Vec<Vec<&str>> = generate_combinations(&fragments, 2).collect();
        
        // Expected combinations for 3 items taken 2 at a time (order matters if it's permutations): 
        // ["a", "b"], ["a", "c"], ["b", "a"], ["b", "c"], ["c", "a"], ["c", "b"]
        assert_eq!(combinations.len(), 6);
        assert!(combinations.contains(&vec!["a", "b"]));
        assert!(combinations.contains(&vec!["b", "c"]));
    }
}
