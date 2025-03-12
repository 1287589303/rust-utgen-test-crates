// Answer 0

#[test]
fn test_is_accel_state_id_less_than_min_accel() {
    let state_id = StateID(1);
    let special = Special {
        max: StateID(10),
        quit_id: StateID(5),
        min_match: StateID(3),
        max_match: StateID(6),
        min_accel: StateID(4),
        max_accel: StateID(8),
        min_start: StateID(2),
        max_start: StateID(9),
    };
    special.is_accel_state(state_id);
}

#[test]
fn test_is_accel_state_id_greater_than_max_accel() {
    let state_id = StateID(9);
    let special = Special {
        max: StateID(10),
        quit_id: StateID(5),
        min_match: StateID(3),
        max_match: StateID(6),
        min_accel: StateID(4),
        max_accel: StateID(8),
        min_start: StateID(2),
        max_start: StateID(9),
    };
    special.is_accel_state(state_id);
}

