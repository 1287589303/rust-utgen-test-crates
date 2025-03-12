// Answer 0

#[test]
fn test_state_id_return_value_min() {
    let state = State {
        id: StateID(0),
        is_match: false,
        ntrans: 1,
        input_ranges: &[0],
        next: &[0],
        pattern_ids: &[0],
        accel: &[0],
    };
    let _ = state.id();
}

#[test]
fn test_state_id_return_value_normal() {
    let state = State {
        id: StateID(1),
        is_match: false,
        ntrans: 2,
        input_ranges: &[0, 1],
        next: &[0, 1],
        pattern_ids: &[0, 1],
        accel: &[0, 1],
    };
    let _ = state.id();
}

#[test]
fn test_state_id_return_value_max() {
    let state = State {
        id: StateID(u32::MAX),
        is_match: true,
        ntrans: 3,
        input_ranges: &[0, 1, 2],
        next: &[0, 1, 2],
        pattern_ids: &[0, 1, 2],
        accel: &[0, 1, 2],
    };
    let _ = state.id();
}

