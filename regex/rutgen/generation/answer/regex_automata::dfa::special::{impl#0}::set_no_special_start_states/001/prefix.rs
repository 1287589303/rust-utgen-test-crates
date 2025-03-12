// Answer 0

#[test]
fn test_set_no_special_start_states_case_1() {
    let mut special = Special {
        max: StateID(5),
        quit_id: StateID(2),
        min_match: StateID(1),
        max_match: StateID(3),
        min_accel: StateID(4),
        max_accel: StateID(6),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    special.set_no_special_start_states();
}

#[test]
fn test_set_no_special_start_states_case_2() {
    let mut special = Special {
        max: StateID(10),
        quit_id: StateID(7),
        min_match: StateID(5),
        max_match: StateID(8),
        min_accel: StateID(9),
        max_accel: StateID(10),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    special.set_no_special_start_states();
}

#[test]
fn test_set_no_special_start_states_case_3() {
    let mut special = Special {
        max: StateID(3),
        quit_id: StateID(1),
        min_match: StateID(0),
        max_match: StateID(3),
        min_accel: StateID(2),
        max_accel: StateID(4),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    special.set_no_special_start_states();
}

#[test]
fn test_set_no_special_start_states_case_4() {
    let mut special = Special {
        max: StateID(6),
        quit_id: StateID(6),
        min_match: StateID(0),
        max_match: StateID(5),
        min_accel: StateID(3),
        max_accel: StateID(5),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    special.set_no_special_start_states();
}

#[test]
fn test_set_no_special_start_states_case_5() {
    let mut special = Special {
        max: StateID(4),
        quit_id: StateID(2),
        min_match: StateID(1),
        max_match: StateID(3),
        min_accel: StateID(0),
        max_accel: StateID(4),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    special.set_no_special_start_states();
}

