// Answer 0

#[test]
fn test_state_match() {
    let state = State::Match;
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

#[test]
fn test_state_fail() {
    let state = State::Fail;
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

#[test]
fn test_state_char() {
    let state = State::Char { target: 1, ch: 'a' };
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

#[test]
fn test_state_ranges() {
    let state = State::Ranges { target: 1, ranges: vec![('a', 'c'), ('d', 'f')] };
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

#[test]
fn test_state_splits() {
    let state = State::Splits { targets: vec![1, 2, 3], reverse: false };
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

#[test]
fn test_state_goto_none() {
    let state = State::Goto { target: 1, look: None };
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

#[test]
fn test_state_goto_some() {
    let state = State::Goto { target: 1, look: Some(Look::Start) };
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

#[test]
fn test_state_capture() {
    let state = State::Capture { target: 1, slot: 0 };
    let mut buffer = String::new();
    let formatter = &mut core::fmt::Formatter::new(&mut buffer);
    state.fmt(formatter).unwrap();
}

