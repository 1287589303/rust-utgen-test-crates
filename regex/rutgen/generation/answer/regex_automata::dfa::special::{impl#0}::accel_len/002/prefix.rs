// Answer 0

#[test]
fn test_accel_len_no_accel_states() {
    let special = Special {
        max: StateID(1),
        quit_id: StateID(2),
        min_match: StateID(3),
        max_match: StateID(4),
        min_accel: DEAD,
        max_accel: DEAD,
        min_start: StateID(5),
        max_start: StateID(6),
    };
    let stride = 1; // any positive integer
    special.accel_len(stride);
}

#[test]
fn test_accel_len_no_accel_states_stride_two() {
    let special = Special {
        max: StateID(1),
        quit_id: StateID(2),
        min_match: StateID(3),
        max_match: StateID(4),
        min_accel: DEAD,
        max_accel: DEAD,
        min_start: StateID(5),
        max_start: StateID(6),
    };
    let stride = 2; // any positive integer
    special.accel_len(stride);
}

#[test]
fn test_accel_len_no_accel_states_stride_ten() {
    let special = Special {
        max: StateID(1),
        quit_id: StateID(2),
        min_match: StateID(3),
        max_match: StateID(4),
        min_accel: DEAD,
        max_accel: DEAD,
        min_start: StateID(5),
        max_start: StateID(6),
    };
    let stride = 10; // any positive integer
    special.accel_len(stride);
}

