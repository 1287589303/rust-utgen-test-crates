// Answer 0

#[test]
fn test_skip_empty_utf8_splits_overlapping_valid_case() {
    let haystack: &[u8] = b"valid haystack";
    let mut state = OverlappingState {
        mat: Some(HalfMatch::new(1, 6)),
        id: Some(0),
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    let input = Input::new(&haystack)
        .anchored(Anchored::No)
        .earliest(false);
    
    let search_fn = |input: &Input, state: &mut OverlappingState| {
        state.mat = Some(HalfMatch::new(1, 7));
        Ok(())
    };

    let _ = skip_empty_utf8_splits_overlapping(&input, &mut state, search_fn);
}

#[test]
fn test_skip_empty_utf8_splits_overlapping_char_boundary() {
    let haystack: &[u8] = b"another valid haystack";
    let mut state = OverlappingState {
        mat: Some(HalfMatch::new(2, 10)),
        id: Some(1),
        at: 0,
        next_match_index: Some(1),
        rev_eoi: false,
    };
    let input = Input::new(&haystack)
        .anchored(Anchored::No)
        .earliest(true);
    
    let search_fn = |input: &Input, state: &mut OverlappingState| {
        state.mat = Some(HalfMatch::new(2, 11));
        Ok(())
    };

    let _ = skip_empty_utf8_splits_overlapping(&input, &mut state, search_fn);
}

#[test]
fn test_skip_empty_utf8_splits_overlapping_long_haystack() {
    let haystack: &[u8] = b"long haystack with multiple characters";
    let mut state = OverlappingState {
        mat: Some(HalfMatch::new(3, 15)),
        id: Some(2),
        at: 0,
        next_match_index: Some(1),
        rev_eoi: false,
    };
    let input = Input::new(&haystack)
        .anchored(Anchored::No)
        .earliest(false);
    
    let search_fn = |input: &Input, state: &mut OverlappingState| {
        state.mat = Some(HalfMatch::new(3, 16));
        Ok(())
    };

    let _ = skip_empty_utf8_splits_overlapping(&input, &mut state, search_fn);
}

#[test]
fn test_skip_empty_utf8_splits_overlapping_empty_input() {
    let haystack: &[u8] = b"";
    let mut state = OverlappingState {
        mat: Some(HalfMatch::new(0, 0)),
        id: Some(0),
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    let input = Input::new(&haystack)
        .anchored(Anchored::No)
        .earliest(false);
    
    let search_fn = |input: &Input, state: &mut OverlappingState| {
        state.mat = Some(HalfMatch::new(0, 0));
        Ok(())
    };

    let _ = skip_empty_utf8_splits_overlapping(&input, &mut state, search_fn);
}

