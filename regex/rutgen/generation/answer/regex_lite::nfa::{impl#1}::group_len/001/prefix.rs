// Answer 0

#[test]
fn test_group_len_empty() {
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
    let _ = nfa.group_len();
}

#[test]
fn test_group_len_one_group() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1"))],
        memory_extra: 0,
    };
    let _ = nfa.group_len();
}

#[test]
fn test_group_len_two_groups() {
    let nfa = NFA {
        pattern: String::from("abc"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1")), Some(Arc::from("group2"))],
        memory_extra: 0,
    };
    let _ = nfa.group_len();
}

#[test]
fn test_group_len_max_groups() {
    let mut cap_index_to_name = Vec::with_capacity(2_usize.pow(32) as usize - 1);
    for i in 0..(2_usize.pow(32) as usize - 1) {
        cap_index_to_name.push(Some(Arc::from(format!("group{}", i))));
    }
    let nfa = NFA {
        pattern: String::from(".*"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra: 0,
    };
    let _ = nfa.group_len();
}

