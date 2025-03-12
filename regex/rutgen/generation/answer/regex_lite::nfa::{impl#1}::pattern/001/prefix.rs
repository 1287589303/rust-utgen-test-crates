// Answer 0

#[test]
fn test_pattern_non_empty() {
    let nfa = NFA {
        pattern: String::from("abc"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.pattern();
}

#[test]
fn test_pattern_empty() {
    let nfa = NFA {
        pattern: String::from(""),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.pattern();
}

#[test]
fn test_pattern_special_characters() {
    let nfa = NFA {
        pattern: String::from("a*b+c?"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.pattern();
}

#[test]
fn test_pattern_whitespace() {
    let nfa = NFA {
        pattern: String::from("   "),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.pattern();
}

#[test]
fn test_pattern_unicode() {
    let nfa = NFA {
        pattern: String::from("こんにちは"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.pattern();
}

#[test]
fn test_pattern_escaped_characters() {
    let nfa = NFA {
        pattern: String::from("a\\b\\c"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let _ = nfa.pattern();
}

