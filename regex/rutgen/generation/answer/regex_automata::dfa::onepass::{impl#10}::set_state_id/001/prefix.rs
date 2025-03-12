// Answer 0

#[test]
fn test_set_state_id_match_wins_true() {
    let mut transition = Transition::new(true, StateID(1), Epsilons(0));
    transition.set_state_id(StateID(100));
}

#[test]
fn test_set_state_id_match_wins_false() {
    let mut transition = Transition::new(false, StateID(2), Epsilons(0));
    transition.set_state_id(StateID(200));
}

#[test]
fn test_set_state_id_boundary_min() {
    let mut transition = Transition::new(false, StateID(0), Epsilons(0));
    transition.set_state_id(StateID(0));
}

#[test]
fn test_set_state_id_boundary_max() {
    let mut transition = Transition::new(true, StateID(0), Epsilons(0));
    transition.set_state_id(StateID(2097151)); // 2^21 - 1
}

#[test]
fn test_set_state_id_epsilons_non_zero() {
    let mut transition = Transition::new(true, StateID(5), Epsilons(123456789));
    transition.set_state_id(StateID(300));
}

