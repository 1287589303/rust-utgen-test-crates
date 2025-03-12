// Answer 0

#[test]
fn test_is_start_state_min_start() {
    let special = Special {
        max: StateID(5),
        quit_id: StateID(1),
        min_match: StateID(2),
        max_match: StateID(3),
        min_accel: StateID(4),
        max_accel: StateID(5),
        min_start: StateID(0),
        max_start: StateID(5),
    };
    let id = StateID(0);
    special.is_start_state(id);
}

#[test]
fn test_is_start_state_max_start() {
    let special = Special {
        max: StateID(5),
        quit_id: StateID(1),
        min_match: StateID(2),
        max_match: StateID(3),
        min_accel: StateID(4),
        max_accel: StateID(5),
        min_start: StateID(0),
        max_start: StateID(5),
    };
    let id = StateID(5);
    special.is_start_state(id);
}

