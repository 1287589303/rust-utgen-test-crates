// Answer 0

#[test]
fn test_find_overlapping_fwd_input_done_no_anchored() {
    let dfa = DFA::always_match().unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new(&[]).span(0..0).anchored(Anchored::No).earliest(false);
    let mut state = OverlappingState { mat: None, id: None, at: 0, next_match_index: None, rev_eoi: false };
    let result = find_overlapping_fwd(&dfa, &mut cache, &input, &mut state);
}

#[test]
fn test_find_overlapping_fwd_input_done_pattern_anchored() {
    let dfa = DFA::always_match().unwrap();
    let mut cache = dfa.create_cache();
    let some_pattern_id = PatternID(0); // assuming a valid pattern id is zero
    let input = Input::new(&[]).span(0..0).anchored(Anchored::Pattern(some_pattern_id)).earliest(false);
    let mut state = OverlappingState { mat: None, id: None, at: 0, next_match_index: None, rev_eoi: false };
    let result = find_overlapping_fwd(&dfa, &mut cache, &input, &mut state);
}

