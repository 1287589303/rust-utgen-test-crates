// Answer 0

#[test]
fn test_match_len_basic_case() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(0),
        min_match: StateID(2),
        max_match: StateID(8),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    let stride = 2;
    special.match_len(stride);
}

#[test]
fn test_match_len_large_stride() {
    let special = Special {
        max: StateID(15),
        quit_id: StateID(0),
        min_match: StateID(3),
        max_match: StateID(10),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    let stride = 5;
    special.match_len(stride);
}

#[test]
fn test_match_len_boundary_case() {
    let special = Special {
        max: StateID(5),
        quit_id: StateID(0),
        min_match: StateID(1),
        max_match: StateID(4),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    let stride = 1;
    special.match_len(stride);
}

#[test]
fn test_match_len_min_match_not_dead() {
    let special = Special {
        max: StateID(20),
        quit_id: StateID(0),
        min_match: StateID(5),
        max_match: StateID(15),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    let stride = 3;
    special.match_len(stride);
}

#[test]
fn test_match_len_min_match_equals_max_match() {
    let special = Special {
        max: StateID(30),
        quit_id: StateID(0),
        min_match: StateID(10),
        max_match: StateID(10),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    let stride = 1;
    special.match_len(stride);
}

