// Answer 0

#[test]
fn test_skip_empty_utf8_splits_overlapping_with_valid_input() {
    let haystack: &[u8] = b"hello";
    let mut overlapping_state = OverlappingState {
        mat: Some(HalfMatch::new(1, 1)),
        id: Some(1),
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    let input = Input::new(haystack)
        .anchored(Anchored::Yes)
        .earliest(true);

    let search_function = |input: &Input, state: &mut OverlappingState| -> Result<(), MatchError> {
        state.mat = Some(HalfMatch::new(1, 1)); // Simulating a search that finds a match
        Ok(())
    };

    let result = skip_empty_utf8_splits_overlapping(&input, &mut overlapping_state, search_function);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_skip_empty_utf8_splits_overlapping_with_different_states() {
    let haystack: &[u8] = b"world";
    let mut overlapping_state = OverlappingState {
        mat: Some(HalfMatch::new(2, 2)),
        id: Some(2),
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    let input = Input::new(haystack)
        .anchored(Anchored::Yes)
        .earliest(false);

    let search_function = |input: &Input, state: &mut OverlappingState| -> Result<(), MatchError> {
        state.mat = Some(HalfMatch::new(2, 2)); // Simulating a search that finds a match
        Ok(())
    };

    let result = skip_empty_utf8_splits_overlapping(&input, &mut overlapping_state, search_function);
    assert_eq!(result, Ok(()));
}

