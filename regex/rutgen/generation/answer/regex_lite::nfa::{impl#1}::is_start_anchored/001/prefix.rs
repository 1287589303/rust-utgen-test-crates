// Answer 0

#[test]
fn test_is_start_anchored_true() {
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.is_start_anchored();
}

#[test]
fn test_is_start_anchored_false() {
    let nfa = NFA {
        pattern: ".*".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.is_start_anchored();
}

#[test]
fn test_is_start_anchored_empty_pattern() {
    let nfa = NFA {
        pattern: "".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.is_start_anchored();
}

#[test]
fn test_is_start_anchored_non_start_anchored_pattern() {
    let nfa = NFA {
        pattern: "xyz".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.is_start_anchored();
}

#[test]
fn test_is_start_anchored_start_anchored_empty_match() {
    let nfa = NFA {
        pattern: "^(a|b|c)".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1"))],
        memory_extra: 0,
    };
    let _ = nfa.is_start_anchored();
}

