// Answer 0

#[test]
fn test_state_builder_empty_new() {
    let state_builder_empty = StateBuilderEmpty::new();
}

#[test]
fn test_state_builder_empty_capacity() {
    let state_builder_empty = StateBuilderEmpty::new();
    let capacity = state_builder_empty.capacity();
}

#[test]
fn test_state_builder_empty_clear() {
    let mut state_builder_empty = StateBuilderEmpty::new();
    state_builder_empty.clear();
}

