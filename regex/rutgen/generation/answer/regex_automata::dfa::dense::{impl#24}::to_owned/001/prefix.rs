// Answer 0

#[test]
fn test_to_owned_non_empty_unique_slices() {
    struct TestDFA;

    impl TestDFA {
        fn new() -> Self {
            TestDFA
        }
    }

    let unique_slices = vec![1u32, 2, 3];
    let pattern_ids = vec![10u32, 20, 30];
    let pattern_len = unique_slices.len();

    let match_states = MatchStates {
        slices: &unique_slices,
        pattern_ids: &pattern_ids,
        pattern_len,
    };

    let owned_match_states = match_states.to_owned();
    let _ = owned_match_states;
}

#[test]
fn test_to_owned_single_element() {
    struct TestDFA;

    impl TestDFA {
        fn new() -> Self {
            TestDFA
        }
    }

    let unique_slices = vec![1u32];
    let pattern_ids = vec![10u32];
    let pattern_len = unique_slices.len();

    let match_states = MatchStates {
        slices: &unique_slices,
        pattern_ids: &pattern_ids,
        pattern_len,
    };

    let owned_match_states = match_states.to_owned();
    let _ = owned_match_states;
}

#[test]
fn test_to_owned_multiple_elements() {
    struct TestDFA;

    impl TestDFA {
        fn new() -> Self {
            TestDFA
        }
    }

    let unique_slices = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    let pattern_ids = vec![10u32, 20u32, 30u32, 40u32, 50u32];
    let pattern_len = unique_slices.len();

    let match_states = MatchStates {
        slices: &unique_slices,
        pattern_ids: &pattern_ids,
        pattern_len,
    };

    let owned_match_states = match_states.to_owned();
    let _ = owned_match_states;
}

#[test]
fn test_to_owned_large_input() {
    struct TestDFA;

    impl TestDFA {
        fn new() -> Self {
            TestDFA
        }
    }

    let unique_slices: Vec<u32> = (1..=1000).collect();
    let pattern_ids: Vec<u32> = (1001..=2000).collect();
    let pattern_len = unique_slices.len();

    let match_states = MatchStates {
        slices: &unique_slices,
        pattern_ids: &pattern_ids,
        pattern_len,
    };

    let owned_match_states = match_states.to_owned();
    let _ = owned_match_states;
}

