// Answer 0

#[test]
fn test_transition_is_dead_when_dead_state() {
    let dead_state_id = StateID::new_unchecked(DEAD.as_usize());
    let transition_dead = Transition::new(true, dead_state_id, Epsilons::default());
    transition_dead.is_dead();
}

#[test]
fn test_transition_is_not_dead_when_alive_state() {
    let alive_state_id = StateID::new_unchecked(1);
    let transition_alive = Transition::new(false, alive_state_id, Epsilons::default());
    transition_alive.is_dead();
}

#[test]
fn test_transition_is_dead_at_boundary_value() {
    let transition_dead_boundary = Transition::new(true, StateID::new_unchecked((1 << Transition::STATE_ID_BITS) - 1), Epsilons::default());
    transition_dead_boundary.is_dead();
}

