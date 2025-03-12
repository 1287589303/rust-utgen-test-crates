// Answer 0

#[test]
fn test_matches_byte_non_zero_transition() {
    let transitions = Box::new([StateID::ZERO; 256]);
    let valid_state_id = StateID(SmallIndex(1));
    let mut transitions_vec = Vec::from(transitions.as_ref());
    transitions_vec[1] = valid_state_id; // Setting a valid transition for byte 1
    let dense_transitions = DenseTransitions {
        transitions: transitions_vec.into_boxed_slice(),
    };
    let _result = dense_transitions.matches_byte(1);
}

#[test]
fn test_matches_byte_another_non_zero_transition() {
    let transitions = Box::new([StateID::ZERO; 256]);
    let valid_state_id = StateID(SmallIndex(2));
    let mut transitions_vec = Vec::from(transitions.as_ref());
    transitions_vec[2] = valid_state_id; // Setting a valid transition for byte 2
    let dense_transitions = DenseTransitions {
        transitions: transitions_vec.into_boxed_slice(),
    };
    let _result = dense_transitions.matches_byte(2);
}

#[test]
fn test_matches_byte_highest_non_zero_transition() {
    let transitions = Box::new([StateID::ZERO; 256]);
    let valid_state_id = StateID(SmallIndex(3));
    let mut transitions_vec = Vec::from(transitions.as_ref());
    transitions_vec[255] = valid_state_id; // Setting a valid transition for byte 255
    let dense_transitions = DenseTransitions {
        transitions: transitions_vec.into_boxed_slice(),
    };
    let _result = dense_transitions.matches_byte(255);
}

