// Answer 0

#[test]
fn test_starts_with_dead_state() {
    let special = Special {
        max: StateID(1),
        quit_id: StateID(1),
        min_match: StateID(1),
        max_match: StateID(2),
        min_accel: StateID(1),
        max_accel: StateID(2),
        min_start: StateID(0), // DEAD state
        max_start: StateID(2),
    };
    special.starts();
}

#[test]
fn test_starts_with_valid_start_state() {
    let special = Special {
        max: StateID(2),
        quit_id: StateID(1),
        min_match: StateID(1),
        max_match: StateID(2),
        min_accel: StateID(1),
        max_accel: StateID(2),
        min_start: StateID(1), // valid start state
        max_start: StateID(2),
    };
    special.starts();
}

#[test]
fn test_starts_with_max_state() {
    let special = Special {
        max: StateID(UINT_MAX),
        quit_id: StateID(1),
        min_match: StateID(1),
        max_match: StateID(UINT_MAX - 1),
        min_accel: StateID(1),
        max_accel: StateID(UINT_MAX - 1),
        min_start: StateID(UINT_MAX), // valid but max start
        max_start: StateID(UINT_MAX),
    };
    special.starts();
} 

#[test]
fn test_starts_with_min_valid_state() {
    let special = Special {
        max: StateID(UINT_MAX),
        quit_id: StateID(1),
        min_match: StateID(1),
        max_match: StateID(UINT_MAX - 1),
        min_accel: StateID(1),
        max_accel: StateID(UINT_MAX - 1),
        min_start: StateID(1), // valid start state
        max_start: StateID(UINT_MAX),
    };
    special.starts();
}

