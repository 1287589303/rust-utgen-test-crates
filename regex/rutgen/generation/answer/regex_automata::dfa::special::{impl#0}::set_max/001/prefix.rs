// Answer 0

#[test]
fn test_set_max_with_all_states_zero() {
    let mut special = Special {
        max: 0,
        quit_id: 0,
        min_match: 0,
        max_match: 0,
        min_accel: 0,
        max_accel: 0,
        min_start: 0,
        max_start: 0,
    };
    special.set_max();
}

#[test]
fn test_set_max_with_various_states() {
    let mut special = Special {
        max: 1,
        quit_id: 2,
        min_match: 0,
        max_match: 1,
        min_accel: 1,
        max_accel: 2,
        min_start: 2,
        max_start: 3,
    };
    special.set_max();
}

#[test]
fn test_set_max_with_large_ids() {
    let mut special = Special {
        max: 100,
        quit_id: 250,
        min_match: 50,
        max_match: 200,
        min_accel: 150,
        max_accel: 300,
        min_start: 200,
        max_start: 400,
    };
    special.set_max();
}

#[test]
fn test_set_max_with_all_states_equal() {
    let mut special = Special {
        max: 10,
        quit_id: 10,
        min_match: 10,
        max_match: 10,
        min_accel: 10,
        max_accel: 10,
        min_start: 10,
        max_start: 10,
    };
    special.set_max();
}

#[test]
fn test_set_max_with_some_states_zero() {
    let mut special = Special {
        max: 5,
        quit_id: 0,
        min_match: 1,
        max_match: 3,
        min_accel: 0,
        max_accel: 4,
        min_start: 2,
        max_start: 0,
    };
    special.set_max();
}

