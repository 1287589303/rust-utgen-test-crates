// Answer 0

#[test]
fn test_overlapping_state_start() {
    let state = OverlappingState::start();
}

#[test]
fn test_overlapping_state_start_properties() {
    let state = OverlappingState::start();
    let mat = state.mat;
    let id = state.id;
    let at = state.at;
    let next_match_index = state.next_match_index;
    let rev_eoi = state.rev_eoi;
}

