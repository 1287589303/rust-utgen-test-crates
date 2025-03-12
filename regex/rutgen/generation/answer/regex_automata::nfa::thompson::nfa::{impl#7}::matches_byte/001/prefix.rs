// Answer 0

#[test]
fn test_matches_byte_zero_transition() {
    let transitions = Box::new([StateID::ZERO; 256]);
    let dense_transitions = DenseTransitions { transitions };

    let result = dense_transitions.matches_byte(0);
}

#[test]
fn test_matches_byte_zero_transition_mid_range() {
    let transitions = Box::new([StateID::ZERO; 256]);
    let dense_transitions = DenseTransitions { transitions };

    let result = dense_transitions.matches_byte(128);
}

#[test]
fn test_matches_byte_zero_transition_max_value() {
    let transitions = Box::new([StateID::ZERO; 256]);
    let dense_transitions = DenseTransitions { transitions };

    let result = dense_transitions.matches_byte(255);
}

