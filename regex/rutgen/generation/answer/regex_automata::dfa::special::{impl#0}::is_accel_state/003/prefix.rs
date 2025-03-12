// Answer 0

#[test]
fn test_is_accel_state_below_min_accel() {
    let special = Special { 
        max: StateID(5), 
        quit_id: StateID(1), 
        min_match: StateID(2), 
        max_match: StateID(3), 
        min_accel: StateID(10), 
        max_accel: StateID(20), 
        min_start: StateID(4), 
        max_start: StateID(6) 
    };
    let id = StateID(9);
    special.is_accel_state(id);
}

#[test]
fn test_is_accel_state_above_max_accel() {
    let special = Special { 
        max: StateID(5), 
        quit_id: StateID(1), 
        min_match: StateID(2), 
        max_match: StateID(3), 
        min_accel: StateID(10), 
        max_accel: StateID(20), 
        min_start: StateID(4), 
        max_start: StateID(6) 
    };
    let id = StateID(21);
    special.is_accel_state(id);
}

#[test]
fn test_is_accel_state_equal_to_min_accel() {
    let special = Special { 
        max: StateID(5), 
        quit_id: StateID(1), 
        min_match: StateID(2), 
        max_match: StateID(3), 
        min_accel: StateID(10), 
        max_accel: StateID(20), 
        min_start: StateID(4), 
        max_start: StateID(6) 
    };
    let id = StateID(10);
    special.is_accel_state(id);
}

#[test]
fn test_is_accel_state_equal_to_max_accel() {
    let special = Special { 
        max: StateID(5), 
        quit_id: StateID(1), 
        min_match: StateID(2), 
        max_match: StateID(3), 
        min_accel: StateID(10), 
        max_accel: StateID(20), 
        min_start: StateID(4), 
        max_start: StateID(6) 
    };
    let id = StateID(20);
    special.is_accel_state(id);
}

