// Answer 0

#[test]
fn test_too_many_states() {
    let error = BuildError::too_many_states();
}

#[test]
fn test_too_many_states_direct_call() {
    let error: BuildError = too_many_states();
}

