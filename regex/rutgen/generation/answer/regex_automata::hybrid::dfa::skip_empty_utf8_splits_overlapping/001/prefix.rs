// Answer 0

#[test]
fn test_skip_empty_utf8_splits_overlapping1() {
    let haystack: &[u8] = b"valid_utf8";
    let match_id = PatternID(0);
    let half_match = HalfMatch::new(match_id, 0); // valid offset
    let state = OverlappingState {
        mat: Some(half_match.clone()),
        id: Some(LazyStateID(0)),
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    let input = Input::new(haystack)
        .anchored(Anchored::Yes)
        .earliest(true);
    
    let mut state = state.clone();
    skip_empty_utf8_splits_overlapping(&input, &mut state, |_, _| Ok(())).unwrap();
}

#[test]
fn test_skip_empty_utf8_splits_overlapping2() {
    let haystack: &[u8] = b"more_valid_utf8";
    let match_id = PatternID(1);
    let half_match = HalfMatch::new(match_id, 5); // valid offset
    let state = OverlappingState {
        mat: Some(half_match.clone()),
        id: Some(LazyStateID(1)),
        at: 0,
        next_match_index: Some(1),
        rev_eoi: false,
    };
    let input = Input::new(haystack)
        .anchored(Anchored::Pattern(match_id))
        .earliest(false);
    
    let mut state = state.clone();
    skip_empty_utf8_splits_overlapping(&input, &mut state, |_, _| Ok(())).unwrap();
}

#[test]
fn test_skip_empty_utf8_splits_overlapping3() {
    let haystack: &[u8] = b"another_valid_utf8";
    let match_id = PatternID(2);
    let half_match = HalfMatch::new(match_id, 1); // valid offset
    let state = OverlappingState {
        mat: Some(half_match.clone()),
        id: Some(LazyStateID(2)),
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    let input = Input::new(haystack)
        .anchored(Anchored::Yes)
        .earliest(true);
    
    let mut state = state.clone();
    skip_empty_utf8_splits_overlapping(&input, &mut state, |_, _| Ok(())).unwrap();
}

