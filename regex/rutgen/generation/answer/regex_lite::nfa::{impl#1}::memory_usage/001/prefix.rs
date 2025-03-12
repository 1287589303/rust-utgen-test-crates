// Answer 0

#[test]
fn test_memory_usage_empty_nfa() {
    let states: Vec<State> = Vec::new();
    let cap_index_to_name: Vec<Option<Arc<str>>> = Vec::new();
    let memory_extra: usize = 0;

    let nfa = NFA {
        pattern: String::from(""),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra,
    };

    let _usage = nfa.memory_usage();
}

#[test]
fn test_memory_usage_single_state_no_captures() {
    let states = vec![State::Char { target: 1, ch: 'a' }];
    let cap_index_to_name: Vec<Option<Arc<str>>> = Vec::new();
    let memory_extra: usize = 0;

    let nfa = NFA {
        pattern: String::from("a"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra,
    };

    let _usage = nfa.memory_usage();
}

#[test]
fn test_memory_usage_multiple_states_no_captures() {
    let states = vec![
        State::Char { target: 1, ch: 'a' },
        State::Char { target: 2, ch: 'b' },
    ];
    let cap_index_to_name: Vec<Option<Arc<str>>> = Vec::new();
    let memory_extra: usize = 0;

    let nfa = NFA {
        pattern: String::from("ab"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra,
    };

    let _usage = nfa.memory_usage();
}

#[test]
fn test_memory_usage_with_captures() {
    let states = vec![State::Capture { target: 1, slot: 0 }];
    let cap_index_to_name = vec![Some(Arc::new(String::from("group1")))]; 
    let memory_extra: usize = 16;

    let nfa = NFA {
        pattern: String::from("(group1)"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra,
    };

    let _usage = nfa.memory_usage();
}

#[test]
fn test_memory_usage_max_states_and_captures() {
    let states = (0..1000).map(|i| State::Char { target: i as StateID, ch: 'a' }).collect::<Vec<State>>();
    let cap_index_to_name = (0..100).map(|i| Some(Arc::new(format!("group{}", i)))).collect::<Vec<Option<Arc<str>>>>();
    let memory_extra: usize = 4096;

    let nfa = NFA {
        pattern: String::from("a".repeat(1000)),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name,
        memory_extra,
    };

    let _usage = nfa.memory_usage();
}

