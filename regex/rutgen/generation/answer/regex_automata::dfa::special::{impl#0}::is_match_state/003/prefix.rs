// Answer 0

#[test]
fn test_is_match_state_min_match_lesser_than_id() {
    let special = Special {
        max: StateID(3),
        quit_id: StateID(5),
        min_match: StateID(4),
        max_match: StateID(6),
        min_accel: StateID(0),
        max_accel: StateID(2),
        min_start: StateID(1),
        max_start: StateID(3),
    };
    let id = StateID(5);
    special.is_match_state(id);
}

#[test]
fn test_is_match_state_id_greater_than_max_match() {
    let special = Special {
        max: StateID(3),
        quit_id: StateID(5),
        min_match: StateID(1),
        max_match: StateID(2),
        min_accel: StateID(0),
        max_accel: StateID(2),
        min_start: StateID(1),
        max_start: StateID(3),
    };
    let id = StateID(3);
    special.is_match_state(id);
}

