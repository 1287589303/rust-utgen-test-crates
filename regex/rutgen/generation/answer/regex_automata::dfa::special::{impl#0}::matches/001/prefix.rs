// Answer 0

#[test]
fn test_matches_with_min_match_less_than_dead() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(5),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(0),
        max_start: StateID(4),
    };
    special.matches();
}

#[test]
fn test_matches_with_min_match_equal_to_dead() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: DEAD,
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(0),
        max_start: StateID(4),
    };
    special.matches();
}

#[test]
fn test_matches_with_min_match_greater_than_dead() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(6),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(0),
        max_start: StateID(4),
    };
    special.matches();
}

#[test]
fn test_matches_with_min_match_at_lower_boundary() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(0),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(0),
        max_start: StateID(4),
    };
    special.matches();
}

#[test]
fn test_matches_with_min_match_at_upper_boundary() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(10),
        max_match: StateID(10),
        min_accel: StateID(2),
        max_accel: StateID(3),
        min_start: StateID(0),
        max_start: StateID(4),
    };
    special.matches();
}

