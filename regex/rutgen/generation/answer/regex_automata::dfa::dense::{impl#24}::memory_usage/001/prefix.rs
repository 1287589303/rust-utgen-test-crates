// Answer 0

#[test]
fn test_memory_usage_empty() {
    let test_data = MatchStates {
        slices: vec![],
        pattern_ids: vec![],
        pattern_len: 0,
    };
    let _ = test_data.memory_usage();
}

#[test]
fn test_memory_usage_one_slice_one_pattern() {
    let test_data = MatchStates {
        slices: vec![1],
        pattern_ids: vec![2],
        pattern_len: 1,
    };
    let _ = test_data.memory_usage();
}

#[test]
fn test_memory_usage_multiple_slices_multiple_patterns() {
    let test_data = MatchStates {
        slices: vec![1, 1, 1],
        pattern_ids: vec![2, 2, 2],
        pattern_len: 3,
    };
    let _ = test_data.memory_usage();
}

#[test]
fn test_memory_usage_large_data() {
    let test_data = MatchStates {
        slices: vec![0; 1000],
        pattern_ids: vec![0; 1000],
        pattern_len: 1000,
    };
    let _ = test_data.memory_usage();
}

