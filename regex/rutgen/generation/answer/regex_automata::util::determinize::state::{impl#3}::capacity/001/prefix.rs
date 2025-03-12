// Answer 0

#[test]
fn test_capacity_empty() {
    let state_builder_empty = StateBuilderEmpty::new();
    let capacity = state_builder_empty.capacity();
}

#[test]
fn test_capacity_zero_size() {
    let state_builder_empty = StateBuilderEmpty(vec![0; 0]);
    let capacity = state_builder_empty.capacity();
}

#[test]
fn test_capacity_small_size() {
    let state_builder_empty = StateBuilderEmpty(vec![0; 5]);
    let capacity = state_builder_empty.capacity();
}

#[test]
fn test_capacity_medium_size() {
    let state_builder_empty = StateBuilderEmpty(vec![0; 10]);
    let capacity = state_builder_empty.capacity();
}

#[test]
fn test_capacity_large_size() {
    let state_builder_empty = StateBuilderEmpty(vec![0; 100]);
    let capacity = state_builder_empty.capacity();
}

#[test]
fn test_capacity_max_usize() {
    let state_builder_empty = StateBuilderEmpty(vec![0; std::usize::MAX]);
    let capacity = state_builder_empty.capacity();
}

