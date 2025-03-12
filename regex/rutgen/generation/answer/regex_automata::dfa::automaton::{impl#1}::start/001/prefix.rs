// Answer 0

#[test]
fn test_overlapping_state_start() {
    let state = OverlappingState::start();
}

#[test]
fn test_overlapping_state_start_properties() {
    let state = OverlappingState::start();
    assert_eq!(state.mat, None);
    assert_eq!(state.id, None);
    assert_eq!(state.at, 0);
    assert_eq!(state.next_match_index, None);
    assert_eq!(state.rev_eoi, false);
}

