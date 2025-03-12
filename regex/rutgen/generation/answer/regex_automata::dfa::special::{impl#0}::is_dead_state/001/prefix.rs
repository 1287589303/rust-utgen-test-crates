// Answer 0

#[test]
fn test_is_dead_state_equal_dead() {
    let state_id = StateID(DEAD);
    let special = Special::new();
    special.is_dead_state(state_id);
}

#[test]
fn test_is_dead_state_less_than_dead() {
    let state_id = StateID(DEAD - 1);
    let special = Special::new();
    special.is_dead_state(state_id);
}

#[test]
fn test_is_dead_state_greater_than_dead() {
    let state_id = StateID(DEAD + 1);
    let special = Special::new();
    special.is_dead_state(state_id);
}

