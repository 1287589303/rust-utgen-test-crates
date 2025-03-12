// Answer 0

#[test]
fn test_is_match_state_dead_state() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(8),
        min_start: StateID(3),
        max_start: StateID(9),
    };
    let id = StateID(DEAD);
    special.is_match_state(id);
}

#[test]
fn test_is_match_state_within_bounds() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(8),
        min_start: StateID(3),
        max_start: StateID(9),
    };
    let id = StateID(6);
    special.is_match_state(id);
}

#[test]
fn test_is_match_state_equal_to_min_match() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(8),
        min_start: StateID(3),
        max_start: StateID(9),
    };
    let id = StateID(5);
    special.is_match_state(id);
}

#[test]
fn test_is_match_state_equal_to_max_match() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(8),
        min_start: StateID(3),
        max_start: StateID(9),
    };
    let id = StateID(10);
    special.is_match_state(id);
}

#[test]
fn test_is_match_state_outside_bounds() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(8),
        min_start: StateID(3),
        max_start: StateID(9),
    };
    let id = StateID(4);
    special.is_match_state(id);
}

#[test]
fn test_is_match_state_above_max_match() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(8),
        min_start: StateID(3),
        max_start: StateID(9),
    };
    let id = StateID(11);
    special.is_match_state(id);
}

