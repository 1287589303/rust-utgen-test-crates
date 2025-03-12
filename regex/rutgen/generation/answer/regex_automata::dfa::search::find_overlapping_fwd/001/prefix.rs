// Answer 0

#[test]
fn test_find_overlapping_fwd_input_done_empty_haystack() {
    let haystack: &[u8] = &[];
    let span = Span::from(0..0);
    let anchored = Anchored::No;
    let earliest = false;

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored)
        .earliest(earliest);

    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_fwd(&my_automaton, &input, &mut state);
    // no assertions; just invoking the function
}

