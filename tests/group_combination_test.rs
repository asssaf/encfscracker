use encfs_cracker::fragment_combination::generate_combinations;
use encfs_cracker::state::Fragment;

#[test]
fn test_group_aware_combinations() {
    let f1 = Fragment {
        text: "a".to_string(),
        group_id: Some("G1".to_string()),
    };
    let f2 = Fragment {
        text: "b".to_string(),
        group_id: Some("G1".to_string()),
    };
    let f3 = Fragment {
        text: "c".to_string(),
        group_id: Some("G2".to_string()),
    };

    let fragments = vec![f1, f2, f3];

    // We want combinations of size 2.
    // Possible combinations of size 2:
    // (a, b) - INVALID (same group G1)
    // (a, c) - VALID
    // (b, a) - INVALID (same group G1)
    // (b, c) - VALID
    // (c, a) - VALID
    // (c, b) - VALID

    let k = 2;
    let combinations: Vec<Vec<Fragment>> = generate_combinations(&fragments, k).collect();

    for combo in combinations {
        let mut groups = std::collections::HashSet::new();
        for f in combo {
            if let Some(gid) = f.group_id {
                assert!(
                    groups.insert(gid),
                    "Each group should appear at most once in a combination"
                );
            }
        }
    }
}
