// Answer 0

#[test]
fn test_get_match_none() {
    let state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };
    state.get_match();
}

#[test]
fn test_get_match_some_offset_0() {
    let state = OverlappingState {
        mat: Some(HalfMatch {
            pattern: PatternID(0), 
            offset: 0,
        }),
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };
    state.get_match();
}

#[test]
fn test_get_match_some_offset_1() {
    let state = OverlappingState {
        mat: Some(HalfMatch {
            pattern: PatternID(1), 
            offset: 1,
        }),
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };
    state.get_match();
}

#[test]
fn test_get_match_some_offset_max() {
    let state = OverlappingState {
        mat: Some(HalfMatch {
            pattern: PatternID(2), 
            offset: std::usize::MAX,
        }),
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };
    state.get_match();
}

