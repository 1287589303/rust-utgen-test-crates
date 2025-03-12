// Answer 0

#[test]
fn test_state_fail_debug() {
    let state = State::Fail;
    let mut output = String::new();
    let result = state.fmt(&mut output);
}

#[test]
fn test_state_match_debug() {
    let state = State::Match;
    let mut output = String::new();
    let result = state.fmt(&mut output);
}

