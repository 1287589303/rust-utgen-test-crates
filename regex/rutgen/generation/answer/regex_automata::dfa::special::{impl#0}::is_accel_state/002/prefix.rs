// Answer 0

#[test]
fn test_is_accel_state_min() {
    let special = Special {
        max: StateID(2),
        quit_id: StateID(3),
        min_match: StateID(4),
        max_match: StateID(5),
        min_accel: StateID(1),
        max_accel: StateID(10),
        min_start: StateID(11),
        max_start: StateID(12),
    };
    let id = StateID(1);
    special.is_accel_state(id);
}

#[test]
fn test_is_accel_state_max() {
    let special = Special {
        max: StateID(2),
        quit_id: StateID(3),
        min_match: StateID(4),
        max_match: StateID(5),
        min_accel: StateID(1),
        max_accel: StateID(10),
        min_start: StateID(11),
        max_start: StateID(12),
    };
    let id = StateID(10);
    special.is_accel_state(id);
}

#[test]
fn test_is_accel_state_inside_range() {
    let special = Special {
        max: StateID(2),
        quit_id: StateID(3),
        min_match: StateID(4),
        max_match: StateID(5),
        min_accel: StateID(1),
        max_accel: StateID(10),
        min_start: StateID(11),
        max_start: StateID(12),
    };
    let id = StateID(5);
    special.is_accel_state(id);
}

