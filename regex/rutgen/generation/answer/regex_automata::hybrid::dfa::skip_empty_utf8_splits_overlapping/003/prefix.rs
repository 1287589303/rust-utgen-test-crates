// Answer 0

#[test]
fn test_skip_empty_utf8_splits_overlapping_with_match() {
    let haystack = b"abcde";
    let input = Input::new(&haystack)
        .anchored(Anchored::No);
    
    let half_match = HalfMatch::new(0, 2);
    let mut state = OverlappingState {
        mat: Some(half_match),
        id: None,
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };

    let search_fn = |_: &Input<'_>, _: &mut OverlappingState| Ok(());

    let result = skip_empty_utf8_splits_overlapping(&input, &mut state, search_fn);
} 

#[test]
fn test_skip_empty_utf8_splits_overlapping_with_boundary() {
    let haystack = b"hello";
    let input = Input::new(&haystack)
        .anchored(Anchored::No);

    let half_match = HalfMatch::new(0, 1);
    let mut state = OverlappingState {
        mat: Some(half_match),
        id: None,
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };

    let search_fn = |_: &Input<'_>, _: &mut OverlappingState| Ok(());

    let result = skip_empty_utf8_splits_overlapping(&input, &mut state, search_fn);
} 

#[test]
fn test_skip_empty_utf8_splits_overlapping_with_edge_case() {
    let haystack = b"world";
    let input = Input::new(&haystack)
        .anchored(Anchored::No);
    
    let half_match = HalfMatch::new(0, 4);
    let mut state = OverlappingState {
        mat: Some(half_match),
        id: None,
        at: 0,
        next_match_index: Some(0),
        rev_eoi: false,
    };

    let search_fn = |_: &Input<'_>, _: &mut OverlappingState| Ok(());

    let result = skip_empty_utf8_splits_overlapping(&input, &mut state, search_fn);
} 

