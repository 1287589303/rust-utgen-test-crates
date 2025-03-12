// Answer 0

#[test]
fn test_is_quit_state_equal_quit_id() {
    let special = Special {
        max: StateID(1),
        quit_id: StateID(5),
        min_match: StateID(2),
        max_match: StateID(3),
        min_accel: StateID(6),
        max_accel: StateID(7),
        min_start: StateID(8),
        max_start: StateID(9),
    };
    let id = StateID(5);
    special.is_quit_state(id);
}

#[test]
fn test_is_quit_state_not_dead() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(15),
        min_match: StateID(1),
        max_match: StateID(2),
        min_accel: StateID(3),
        max_accel: StateID(4),
        min_start: StateID(5),
        max_start: StateID(6),
    };
    let id = StateID(15);
    special.is_quit_state(id);
}

#[test]
fn test_is_quit_state_not_equal_to_dead_or_quit() {
    let special = Special {
        max: StateID(20),
        quit_id: StateID(25),
        min_match: StateID(5),
        max_match: StateID(6),
        min_accel: StateID(7),
        max_accel: StateID(8),
        min_start: StateID(9),
        max_start: StateID(10),
    };
    let id = StateID(30);
    special.is_quit_state(id);
}

