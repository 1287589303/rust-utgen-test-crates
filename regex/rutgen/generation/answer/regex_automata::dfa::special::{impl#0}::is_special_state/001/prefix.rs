// Answer 0

#[test]
fn test_is_special_state_with_id_less_than_max() {
    let special = Special { max: StateID(5), quit_id: StateID(1), min_match: StateID(2), max_match: StateID(3), min_accel: StateID(4), max_accel: StateID(5), min_start: StateID(6), max_start: StateID(7) };
    let id = StateID(4);
    special.is_special_state(id);
}

#[test]
fn test_is_special_state_with_id_equal_to_max() {
    let special = Special { max: StateID(5), quit_id: StateID(1), min_match: StateID(2), max_match: StateID(3), min_accel: StateID(4), max_accel: StateID(5), min_start: StateID(6), max_start: StateID(7) };
    let id = StateID(5);
    special.is_special_state(id);
}

#[test]
fn test_is_special_state_with_id_greater_than_max() {
    let special = Special { max: StateID(5), quit_id: StateID(1), min_match: StateID(2), max_match: StateID(3), min_accel: StateID(4), max_accel: StateID(5), min_start: StateID(6), max_start: StateID(7) };
    let id = StateID(6);
    special.is_special_state(id);
}

#[test]
fn test_is_special_state_with_id_at_zero() {
    let special = Special { max: StateID(5), quit_id: StateID(1), min_match: StateID(2), max_match: StateID(3), min_accel: StateID(4), max_accel: StateID(5), min_start: StateID(6), max_start: StateID(7) };
    let id = StateID(0);
    special.is_special_state(id);
}

#[test]
fn test_is_special_state_with_max_at_zero() {
    let special = Special { max: StateID(0), quit_id: StateID(1), min_match: StateID(2), max_match: StateID(3), min_accel: StateID(4), max_accel: StateID(5), min_start: StateID(6), max_start: StateID(7) };
    let id = StateID(0);
    special.is_special_state(id);

    let id_greater = StateID(1);
    special.is_special_state(id_greater);
}

