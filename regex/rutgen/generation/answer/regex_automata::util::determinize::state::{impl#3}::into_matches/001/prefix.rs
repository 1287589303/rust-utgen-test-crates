// Answer 0

#[test]
fn test_into_matches_empty_state_builder() {
    let state_builder_empty = StateBuilderEmpty::new();
    let state_builder_matches = state_builder_empty.into_matches();
}

#[test]
fn test_into_matches_non_empty_state_builder() {
    let mut state_builder_empty = StateBuilderEmpty(vec![1, 2, 3]);
    let state_builder_matches = state_builder_empty.into_matches();
}

#[test]
fn test_into_matches_capacity() {
    let state_builder_empty = StateBuilderEmpty(vec![0; 5]);
    let state_builder_matches = state_builder_empty.into_matches();
}

