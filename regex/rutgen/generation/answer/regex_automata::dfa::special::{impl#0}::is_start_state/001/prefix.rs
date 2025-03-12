// Answer 0

#[test]
fn test_is_start_state_dead_id() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(2),
        max_match: StateID(3),
        min_accel: StateID(4),
        max_accel: StateID(5),
        min_start: StateID(6),
        max_start: StateID(9),
    };
    let dead_id = DEAD;
    special.is_start_state(dead_id);
}

#[test]
fn test_is_start_state_below_min_start() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(2),
        max_match: StateID(3),
        min_accel: StateID(4),
        max_accel: StateID(5),
        min_start: StateID(6),
        max_start: StateID(9),
    };
    let below_min_start = StateID(5);
    special.is_start_state(below_min_start);
}

#[test]
fn test_is_start_state_above_max_start() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(1),
        min_match: StateID(2),
        max_match: StateID(3),
        min_accel: StateID(4),
        max_accel: StateID(5),
        min_start: StateID(6),
        max_start: StateID(9),
    };
    let above_max_start = StateID(10);
    special.is_start_state(above_max_start);
}

