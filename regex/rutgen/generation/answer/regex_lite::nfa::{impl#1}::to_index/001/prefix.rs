// Answer 0

#[test]
fn test_to_index_valid_capture_name() {
    let cap_name_to_index = std::collections::HashMap::from([
        (Arc::from("group1"), 0),
        (Arc::from("group2"), 1),
    ]);
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index,
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    nfa.to_index("group1");
}

#[test]
fn test_to_index_non_existent_capture_name() {
    let cap_name_to_index = std::collections::HashMap::from([
        (Arc::from("existing"), 0),
    ]);
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index,
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    nfa.to_index("non_existent");
}

#[test]
fn test_to_index_empty_string() {
    let cap_name_to_index = std::collections::HashMap::from([
        (Arc::from(""), 0),
    ]);
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index,
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    nfa.to_index("");
}

#[test]
fn test_to_index_special_characters() {
    let cap_name_to_index = std::collections::HashMap::from([
        (Arc::from("group_with_specials!@#"), 0),
    ]);
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index,
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    nfa.to_index("group_with_specials!@#");
}

#[test]
fn test_to_index_non_ascii_characters() {
    let cap_name_to_index = std::collections::HashMap::from([
        (Arc::from("グループ"), 0),
    ]);
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index,
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    nfa.to_index("グループ");
}

#[test]
fn test_to_index_very_long_name() {
    let long_name = "a".repeat(255);
    let cap_name_to_index = std::collections::HashMap::from([
        (Arc::from(&long_name), 0),
    ]);
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index,
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    nfa.to_index(&long_name);
}

