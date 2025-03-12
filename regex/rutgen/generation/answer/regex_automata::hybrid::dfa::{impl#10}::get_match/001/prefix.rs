// Answer 0

#[test]
fn test_get_match_none() {
    let state = OverlappingState::start();
    let match_result = state.get_match();
}

#[test]
fn test_get_match_some() {
    let half_match = HalfMatch {
        pattern: PatternID(1),
        offset: 0,
    };
    let state = OverlappingState {
        mat: Some(half_match),
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };
    let match_result = state.get_match();
}

#[test]
fn test_get_match_boundary_offset() {
    let half_match = HalfMatch {
        pattern: PatternID(2),
        offset: 10,
    };
    let state = OverlappingState {
        mat: Some(half_match),
        id: None,
        at: 10,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    let match_result = state.get_match();
}

#[test]
fn test_get_match_max_offset() {
    let half_match = HalfMatch {
        pattern: PatternID(3),
        offset: usize::MAX,
    };
    let state = OverlappingState {
        mat: Some(half_match),
        id: None,
        at: usize::MAX,
        next_match_index: Some(1),
        rev_eoi: false,
    };
    let match_result = state.get_match();
}

