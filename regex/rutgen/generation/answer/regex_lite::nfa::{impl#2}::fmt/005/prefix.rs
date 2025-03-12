// Answer 0

#[test]
fn test_nfa_with_non_empty_pattern_and_single_state() {
    let pattern = String::from("a");
    let states = vec![State::Match];
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name = Vec::new();
    let memory_extra: usize = 0;

    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored,
        is_match_empty,
        static_explicit_captures_len,
        cap_name_to_index,
        cap_index_to_name,
        memory_extra,
    };

    let _ = format!("{:?}", nfa);
}

#[test]
fn test_nfa_with_non_empty_pattern_and_multiple_states() {
    let pattern = String::from("abc");
    let states = vec![
        State::Char { target: 1, ch: 'a' },
        State::Char { target: 2, ch: 'b' },
        State::Char { target: 3, ch: 'c' },
        State::Match,
    ];
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name = Vec::new();
    let memory_extra: usize = 0;

    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored,
        is_match_empty,
        static_explicit_captures_len,
        cap_name_to_index,
        cap_index_to_name,
        memory_extra,
    };

    let _ = format!("{:?}", nfa);
}

#[test]
fn test_nfa_with_single_character_pattern_and_fail_state() {
    let pattern = String::from("z");
    let states = vec![State::Fail];
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name = Vec::new();
    let memory_extra: usize = 0;

    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored,
        is_match_empty,
        static_explicit_captures_len,
        cap_name_to_index,
        cap_index_to_name,
        memory_extra,
    };

    let _ = format!("{:?}", nfa);
}

#[test]
fn test_nfa_with_empty_state_list() {
    let pattern = String::from("regex");
    let states: Vec<State> = vec![];
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name = Vec::new();
    let memory_extra: usize = 0;

    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored,
        is_match_empty,
        static_explicit_captures_len,
        cap_name_to_index,
        cap_index_to_name,
        memory_extra,
    };

    let _ = format!("{:?}", nfa);
}

