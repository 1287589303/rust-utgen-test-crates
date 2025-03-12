// Answer 0

#[test]
fn test_state_fail_debug() {
    let state = State::Fail;
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state);
}

#[test]
fn test_state_fail_fmt() {
    let state = State::Fail;
    let mut output = String::new();
    let _ = state.fmt(&mut output);
}

