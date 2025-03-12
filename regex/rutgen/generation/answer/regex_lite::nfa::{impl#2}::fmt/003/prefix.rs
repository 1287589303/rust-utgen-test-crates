// Answer 0

#[test]
fn test_nfa_fmt_valid() {
    let pattern = "abc".to_string();
    let states = vec![State::Char { target: 1, ch: 'a' }];
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name = vec![];
    let memory_extra = 0;

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

    let mut f = core::fmt::Formatter::default();
    let _ = nfa.fmt(&mut f);
}

#[test]
fn test_nfa_fmt_err_on_state_repr() {
    let pattern = "abc".to_string();
    let states = vec![State::Fail];
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name = vec![];
    let memory_extra = 0;

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

    let mut f = core::fmt::Formatter::default();
    let _ = nfa.fmt(&mut f);
}

#[test]
fn test_nfa_fmt_empty_states() {
    let pattern = "abc".to_string();
    let states = vec![];
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name = vec![];
    let memory_extra = 0;

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

    let mut f = core::fmt::Formatter::default();
    let _ = nfa.fmt(&mut f);
}

