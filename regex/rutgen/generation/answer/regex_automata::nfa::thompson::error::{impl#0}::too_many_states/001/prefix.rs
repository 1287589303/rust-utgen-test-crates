// Answer 0

#[test]
fn test_too_many_states_zero() {
    let given = 0;
    let error = BuildError::too_many_states(given);
}

#[test]
fn test_too_many_states_limit() {
    let given = StateID::LIMIT;
    let error = BuildError::too_many_states(given);
}

#[test]
fn test_too_many_states_over_limit() {
    let given = StateID::LIMIT + 1;
    let error = BuildError::too_many_states(given);
}

