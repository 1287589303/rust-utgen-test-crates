// Answer 0

#[test]
fn test_accels_with_min_accel_zero() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(5),
        min_match: StateID(2),
        max_match: StateID(4),
        min_accel: StateID(0),
        max_accel: StateID(1),
        min_start: StateID(3),
        max_start: StateID(6),
    };
    special.accels();
}

#[test]
fn test_accels_with_min_accel_non_dead() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(5),
        min_match: StateID(2),
        max_match: StateID(4),
        min_accel: StateID(1),
        max_accel: StateID(3),
        min_start: StateID(3),
        max_start: StateID(6),
    };
    special.accels();
}

#[test]
fn test_accels_with_min_accel_equals_dead() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(5),
        min_match: StateID(2),
        max_match: StateID(4),
        min_accel: DEAD,
        max_accel: StateID(3),
        min_start: StateID(3),
        max_start: StateID(6),
    };
    special.accels();
}

#[test]
fn test_accels_with_min_accel_greater_than_dead() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(5),
        min_match: StateID(2),
        max_match: StateID(4),
        min_accel: StateID(1),
        max_accel: StateID(4),
        min_start: StateID(3),
        max_start: StateID(6),
    };
    special.accels();
}

