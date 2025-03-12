// Answer 0

#[test]
fn test_is_match_state_min_match() {
    let state_id = StateID(5);
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: state_id,
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(4),
        max_start: StateID(8),
    };
    special.is_match_state(state_id);
}

#[test]
fn test_is_match_state_within_bounds() {
    let state_id = StateID(7);
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(4),
        max_start: StateID(8),
    };
    special.is_match_state(state_id);
}

#[test]
fn test_is_match_state_at_max_match() {
    let state_id = StateID(10);
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: state_id,
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(4),
        max_start: StateID(8),
    };
    special.is_match_state(state_id);
}

