// Answer 0

#[test]
fn test_builder_new() {
    let builder = Builder::new();
    assert!(builder.pattern_id.is_none());
    assert!(builder.states.is_empty());
    assert!(builder.start_pattern.is_empty());
    assert!(builder.captures.is_empty());
    assert_eq!(builder.memory_states, 0);
    assert!(!builder.utf8);
    assert!(!builder.reverse);
    assert_eq!(builder.look_matcher.lineterm, DebugByte::default());
    assert!(builder.size_limit.is_none());
}

