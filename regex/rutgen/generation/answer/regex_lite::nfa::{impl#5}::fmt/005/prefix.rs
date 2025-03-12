// Answer 0

#[test]
fn test_fmt_char_state() {
    let state = State::Char { target: 1, ch: 'a' };
    let _ = format!("{:?}", state);
}

#[test]
fn test_fmt_goto_state_with_look() {
    let state = State::Goto { target: 2, look: Some(Look::Start) };
    let _ = format!("{:?}", state);
}

#[test]
fn test_fmt_goto_state_without_look() {
    let state = State::Goto { target: 3, look: None };
    let _ = format!("{:?}", state);
}

