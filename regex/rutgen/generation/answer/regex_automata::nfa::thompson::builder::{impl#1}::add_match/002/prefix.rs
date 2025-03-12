// Answer 0

#[test]
fn test_add_match_with_valid_pattern_id_and_no_state_limit() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.pattern_id = Some(pattern_id);
    builder.memory_states = 0;
    builder.set_size_limit(None).unwrap();
    let state_id = builder.add_match().unwrap();
}

#[test]
fn test_add_match_with_valid_pattern_id_and_state_limit() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.pattern_id = Some(pattern_id);
    builder.memory_states = 0;
    builder.set_size_limit(Some(100)).unwrap();
    let state_id = builder.add_match().unwrap();
}

#[test]
fn test_add_match_with_multiple_states() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.pattern_id = Some(pattern_id);
    
    for _ in 0..5 {
        builder.add_match().unwrap();
    }
    
    let state_id = builder.add_match().unwrap();
}

#[test]
fn test_add_match_with_state_limit_edge_case() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.pattern_id = Some(pattern_id);
    builder.memory_states = 0;
    builder.set_size_limit(Some(1)).unwrap();
    
    // Test adding one match after setting a limit.
    let _ = builder.add_match().unwrap();
    
    // Test that an additional match exceeds the limit.
    builder.memory_states = 1; // Simulating having one state already.
    let result = builder.add_match();
    assert!(result.is_err());
}

#[test]
fn test_add_match_exceed_state_identifier() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.pattern_id = Some(pattern_id);
    builder.set_size_limit(None).unwrap();
    
    // Fill state identifier space to its maximum.
    let max_states = SmallIndex::MAX as usize;
    for _ in 0..max_states {
        builder.add_match().unwrap();
    }
    
    // One more match should exceed the state identifier limit.
    let result = builder.add_match();
    assert!(result.is_err());
}

