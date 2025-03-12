// Answer 0

#[test]
fn test_builder_new() {
    let builder = Builder::new();
}

#[test]
fn test_builder_new_config() {
    let builder = Builder::new();
    assert_eq!(builder.pattern_id, None);
    assert_eq!(builder.states.len(), 0);
    assert_eq!(builder.start_pattern.len(), 0);
    assert_eq!(builder.captures.len(), 0);
    assert_eq!(builder.memory_states, 0);
    assert_eq!(builder.utf8, true);
    assert_eq!(builder.reverse, false);
    // assume LookMatcher can be created as below
    let look_matcher = LookMatcher::new(); // hypothetical initialization
    assert_eq!(builder.look_matcher, look_matcher);
    assert_eq!(builder.size_limit, None);
}

