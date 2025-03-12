// Answer 0

#[test]
fn test_captures_iter_empty_haystack() {
    let nfa = NFA {
        pattern: "ab".to_string(),
        states: vec![],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let cache = CachePoolGuard::default();
    let haystack: &[u8] = b"";
    let _captures = pikevm.captures_iter(cache, haystack);
}

#[test]
fn test_captures_iter_single_byte_haystack() {
    let nfa = NFA {
        pattern: "a".to_string(),
        states: vec![],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let cache = CachePoolGuard::default();
    let haystack: &[u8] = b"a";
    let _captures = pikevm.captures_iter(cache, haystack);
}

#[test]
fn test_captures_iter_multiple_bytes_haystack() {
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1")), Some(Arc::from("group2"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let cache = CachePoolGuard::default();
    let haystack: &[u8] = b"abcde";
    let _captures = pikevm.captures_iter(cache, haystack);
}

#[test]
fn test_captures_iter_large_haystack() {
    let nfa = NFA {
        pattern: "xyz".to_string(),
        states: vec![],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let cache = CachePoolGuard::default();
    let haystack: &[u8] = b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxyzyzyz";
    let _captures = pikevm.captures_iter(cache, haystack);
}

#[test]
fn test_captures_iter_max_boundary_haystack() {
    let nfa = NFA {
        pattern: "pattern".to_string(),
        states: vec![],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1"))],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
    let cache = CachePoolGuard::default();
    let haystack: &[u8] = &[b'a'; 1024];
    let _captures = pikevm.captures_iter(cache, haystack);
}

