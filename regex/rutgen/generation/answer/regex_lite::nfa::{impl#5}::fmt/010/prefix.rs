// Answer 0

#[test]
fn test_state_splits_empty_targets_non_reverse() {
    let targets: Vec<StateID> = Vec::new();
    let reverse = false;
    let state = State::Splits { targets, reverse };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", state);
}

#[test]
fn test_state_splits_empty_targets_reverse() {
    let targets: Vec<StateID> = Vec::new();
    let reverse = true;
    let state = State::Splits { targets, reverse };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", state);
}

