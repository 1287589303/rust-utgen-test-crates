// Answer 0

#[test]
fn test_capture_names_empty() {
    let cap_index_to_name: Vec<Option<Arc<String>>> = vec![];
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra: 0,
    };
    let _capture_names = nfa.capture_names();
}

#[test]
fn test_capture_names_single() {
    let cap_index_to_name: Vec<Option<Arc<String>>> = vec![Some(Arc::new("group1".to_string()))];
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra: 0,
    };
    let _capture_names = nfa.capture_names();
}

#[test]
fn test_capture_names_maximum() {
    let cap_index_to_name: Vec<Option<Arc<String>>> = (0..N).map(|i| Some(Arc::new(format!("group{}", i)))).collect();
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(N),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra: 0,
    };
    let _capture_names = nfa.capture_names();
}

#[test]
fn test_capture_names_mixed() {
    let cap_index_to_name: Vec<Option<Arc<String>>> = vec![
        Some(Arc::new("group1".to_string())),
        None,
        Some(Arc::new("group3".to_string())),
    ];
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(3),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra: 0,
    };
    let _capture_names = nfa.capture_names();
}

#[test]
fn test_capture_names_std_feature() {
    let cap_index_to_name: Vec<Option<Arc<String>>> = vec![Some(Arc::new("group1".to_string()))];
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra: 0,
    };
    let _capture_names = nfa.capture_names();
}

#[test]
#[cfg(not(feature = "std"))]
fn test_capture_names_no_std_feature() {
    let cap_index_to_name: Vec<Option<Arc<String>>> = vec![];
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra: 0,
    };
    let _capture_names = nfa.capture_names();
}

