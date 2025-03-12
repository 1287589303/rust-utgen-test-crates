// Answer 0

#[test]
fn test_fmt_with_empty_states_and_valid_pattern() {
    let pattern = String::from("test");
    let states: Vec<State> = Vec::new();
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name: Vec<Option<Arc<str>>> = Vec::new();
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

    let mut buffer = Vec::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    let _ = nfa.fmt(formatter);
}

#[test]
fn test_fmt_with_err_on_closing_bracket() {
    let pattern = String::from("test");
    let states: Vec<State> = Vec::new();
    let start: StateID = 0;
    let is_start_anchored = false;
    let is_match_empty = false;
    let static_explicit_captures_len = None;
    let cap_name_to_index = CaptureNameMap::new();
    let cap_index_to_name: Vec<Option<Arc<str>>> = Vec::new();
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

    let mut invalid_buffer = alloc::vec![0]; // Create a buffer that causes writeln! to fail
    let formatter = &mut core::fmt::Formatter::new(&mut invalid_buffer);
    let _ = nfa.fmt(formatter);
}

