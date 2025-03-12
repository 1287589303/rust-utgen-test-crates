// Answer 0

#[test]
fn test_state_fmt_goto_with_look_some() {
    let target: u32 = 5;
    let look = crate::nfa::Look::Start;
    let state = crate::nfa::State::Goto { target, look: Some(look) };
    let mut output = core::fmt::Formatter::default();
    let _ = state.fmt(&mut output);
}

#[test]
fn test_state_fmt_goto_with_look_none() {
    let target: u32 = 3;
    let state = crate::nfa::State::Goto { target, look: None };
    let mut output = core::fmt::Formatter::default();
    let _ = state.fmt(&mut output);
}

#[test]
fn test_state_fmt_ranges() {
    let target: u32 = 10;
    let ranges = vec![('a', 'z'), ('A', 'Z')];
    let state = crate::nfa::State::Ranges { target, ranges };
    let mut output = core::fmt::Formatter::default();
    let _ = state.fmt(&mut output);
}

