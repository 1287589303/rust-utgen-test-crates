// Answer 0

#[test]
fn test_is_accel_state_boundary_conditions_min() {
    struct TestDFA {
        special: Special,
    }

    let dfa = TestDFA {
        special: Special {
            max: 10,
            quit_id: 1,
            min_match: 2,
            max_match: 5,
            min_accel: 3,
            max_accel: 7,
            min_start: 4,
            max_start: 9,
        },
    };

    let _ = dfa.is_accel_state(StateID(0));
    let _ = dfa.is_accel_state(StateID(2));
}

#[test]
fn test_is_accel_state_boundary_conditions_min_accel() {
    struct TestDFA {
        special: Special,
    }

    let dfa = TestDFA {
        special: Special {
            max: 10,
            quit_id: 1,
            min_match: 2,
            max_match: 5,
            min_accel: 3,
            max_accel: 7,
            min_start: 4,
            max_start: 9,
        },
    };

    let _ = dfa.is_accel_state(StateID(3));
}

#[test]
fn test_is_accel_state_boundary_conditions_max_accel() {
    struct TestDFA {
        special: Special,
    }

    let dfa = TestDFA {
        special: Special {
            max: 10,
            quit_id: 1,
            min_match: 2,
            max_match: 5,
            min_accel: 3,
            max_accel: 7,
            min_start: 4,
            max_start: 9,
        },
    };

    let _ = dfa.is_accel_state(StateID(7));
}

#[test]
fn test_is_accel_state_boundary_conditions_max() {
    struct TestDFA {
        special: Special,
    }

    let dfa = TestDFA {
        special: Special {
            max: 10,
            quit_id: 1,
            min_match: 2,
            max_match: 5,
            min_accel: 3,
            max_accel: 7,
            min_start: 4,
            max_start: 9,
        },
    };

    let _ = dfa.is_accel_state(StateID(10));
}

#[test]
fn test_is_accel_state_boundary_conditions_just_outside_range() {
    struct TestDFA {
        special: Special,
    }

    let dfa = TestDFA {
        special: Special {
            max: 10,
            quit_id: 1,
            min_match: 2,
            max_match: 5,
            min_accel: 3,
            max_accel: 7,
            min_start: 4,
            max_start: 9,
        },
    };

    let _ = dfa.is_accel_state(StateID(2));
    let _ = dfa.is_accel_state(StateID(8));
}

