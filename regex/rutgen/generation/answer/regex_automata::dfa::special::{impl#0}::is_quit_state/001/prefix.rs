// Answer 0

#[test]
fn test_is_quit_state_with_valid_quit_id() {
    let special = Special {
        max: StateID(1),
        quit_id: StateID(2),
        min_match: StateID(3),
        max_match: StateID(4),
        min_accel: StateID(5),
        max_accel: StateID(6),
        min_start: StateID(7),
        max_start: StateID(8),
    };
    let id = special.quit_id;
    special.is_quit_state(id);
}

#[test]
fn test_is_quit_state_with_zero_quit_id() {
    let special = Special {
        max: StateID(1),
        quit_id: StateID(0),
        min_match: StateID(3),
        max_match: StateID(4),
        min_accel: StateID(5),
        max_accel: StateID(6),
        min_start: StateID(7),
        max_start: StateID(8),
    };
    let id = special.quit_id;
    special.is_quit_state(id);
}

#[test]
fn test_is_quit_state_with_non_dead_state_equal_quit_id() {
    let special = Special {
        max: StateID(10),
        quit_id: StateID(12),
        min_match: StateID(13),
        max_match: StateID(14),
        min_accel: StateID(15),
        max_accel: StateID(16),
        min_start: StateID(17),
        max_start: StateID(18),
    };
    let id = special.quit_id;
    special.is_quit_state(id);
}

