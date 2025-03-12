// Answer 0

#[test]
fn test_skip_empty_utf8_splits_overlapping() {
    let haystack = "你好世界".as_bytes(); // Multi-byte UTF-8 characters
    let initial_state = OverlappingState::start();
    let mut state = initial_state;

    let input = Input::new(&haystack)
        .anchored(Anchored::No);

    let match_offset = 2; // Not on a character boundary
    let pattern_id = 1; // Example PatternID

    // Simulated half-match
    let half_match = HalfMatch::new(pattern_id, match_offset);
    state.mat = Some(half_match); // Precondition for get_match() to return Some

    let search = |input: &Input, state: &mut OverlappingState| {
        state.mat = Some(HalfMatch::new(pattern_id, match_offset + 1)); // Modify state and simulate a match
        Ok(())
    };

    skip_empty_utf8_splits_overlapping(&input, &mut state, search).unwrap();
}

#[test]
fn test_skip_empty_utf8_splits_overlapping_with_boundary() {
    let haystack = "こんにちは".as_bytes(); // Multi-byte UTF-8 characters
    let initial_state = OverlappingState::start();
    let mut state = initial_state;

    let input = Input::new(&haystack)
        .anchored(Anchored::No);

    let match_offset = 3; // Starting in the middle of a multi-byte character (not a boundary)
    let pattern_id = 2; // Another example PatternID

    // Simulated half-match
    let half_match = HalfMatch::new(pattern_id, match_offset);
    state.mat = Some(half_match); // Precondition for get_match() to return Some

    let search = |input: &Input, state: &mut OverlappingState| {
        state.mat = Some(HalfMatch::new(pattern_id, match_offset + 1)); // Continue searching to maintain valid state
        Ok(())
    };

    skip_empty_utf8_splits_overlapping(&input, &mut state, search).unwrap();
}

