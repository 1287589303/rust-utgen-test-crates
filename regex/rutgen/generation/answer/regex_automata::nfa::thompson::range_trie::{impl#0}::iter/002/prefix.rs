// Answer 0

#[test]
fn test_iter_with_empty_transitions() {
    let mut trie = RangeTrie::new();
    let f = |_: &[Utf8Range]| -> Result<(), ()> { Ok(()) };
    
    // Invoke iter with empty transitions
    let result = trie.iter(f);
    assert!(result.is_ok());
}

#[test]
fn test_iter_with_single_transition() {
    let mut trie = RangeTrie::new();
    let f = |_: &[Utf8Range]| -> Result<(), ()> { Ok(()) };

    // Setup a state with a single transition
    let state_id = trie.add_empty();
    trie.add_transition(state_id, Utf8Range::new(0x61..=0x61), FINAL); // 'a' transition
    let result = trie.iter(f);
    assert!(result.is_ok());
}

#[test]
fn test_iter_with_multiple_transitions() {
    let mut trie = RangeTrie::new();
    let f = |_: &[Utf8Range]| -> Result<(), ()> { Ok(()) };

    // Setup state with multiple transitions
    let state_id = trie.add_empty();
    trie.add_transition(state_id, Utf8Range::new(0x61..=0x62), FINAL); // 'a' and 'b' transition
    trie.add_transition(state_id, Utf8Range::new(0x63..=0x63), FINAL); // 'c' transition
    let result = trie.iter(f);
    assert!(result.is_ok());
}

#[test]
fn test_iter_with_last_transition() {
    let mut trie = RangeTrie::new();
    let f = |_: &[Utf8Range]| -> Result<(), ()> { Ok(()) };

    // Setup a state with multiple transitions where tidx equals length of transitions minus one
    let state_id = trie.add_empty();
    trie.add_transition(state_id, Utf8Range::new(0x61..=0x61), FINAL); // 'a'
    trie.add_transition(state_id, Utf8Range::new(0x62..=0x62), FINAL); // 'b'
    let result = trie.iter(f);
    assert!(result.is_ok());
}

#[test]
fn test_iter_with_ranges() {
    let mut trie = RangeTrie::new();
    let f = |ranges: &[Utf8Range]| -> Result<(), ()> { 
        assert!(!ranges.is_empty());
        Ok(()) 
    };

    // Setup a state with valid Utf8Ranges
    let state_id = trie.add_empty();
    trie.add_transition(state_id, Utf8Range::new(0x61..=0x63), FINAL); // 'a', 'b', 'c' transition
    let result = trie.iter(f);
    assert!(result.is_ok());
}

