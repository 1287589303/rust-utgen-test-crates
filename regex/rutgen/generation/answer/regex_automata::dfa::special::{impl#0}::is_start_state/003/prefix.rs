// Answer 0

#[test]
fn test_is_start_state_min_boundary() {
    let special = Special {
        max: StateID(5),
        quit_id: StateID(1),
        min_match: StateID(2),
        max_match: StateID(4),
        min_accel: StateID(2),
        max_accel: StateID(5),
        min_start: StateID(3),
        max_start: StateID(7),
    };

    let id = StateID(4);
    special.is_start_state(id);
}

#[test]
fn test_is_start_state_max_boundary() {
    let special = Special {
        max: StateID(8),
        quit_id: StateID(2),
        min_match: StateID(3),
        max_match: StateID(6),
        min_accel: StateID(3),
        max_accel: StateID(8),
        min_start: StateID(3),
        max_start: StateID(7),
    };

    let id = StateID(6);
    special.is_start_state(id);
}

#[test]
fn test_is_start_state_valid_case() {
    let special = Special {
        max: StateID(9),
        quit_id: StateID(2),
        min_match: StateID(4),
        max_match: StateID(5),
        min_accel: StateID(4),
        max_accel: StateID(9),
        min_start: StateID(3),
        max_start: StateID(8),
    };

    let id = StateID(5);
    special.is_start_state(id);
}

#[test]
fn test_is_start_state_above_min_below_max() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(3),
        min_match: StateID(5),
        max_match: StateID(7),
        min_accel: StateID(5),
        max_accel: StateID(10),
        min_start: StateID(3),
        max_start: StateID(9),
    };

    let id = StateID(7);
    special.is_start_state(id);
}

#[test]
fn test_is_start_state_invalid_case() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(4),
        min_match: StateID(5),
        max_match: StateID(6),
        min_accel: StateID(1),
        max_accel: StateID(10),
        min_start: StateID(5),
        max_start: StateID(9),
    };

    let id = StateID(4);
    special.is_start_state(id);
}

