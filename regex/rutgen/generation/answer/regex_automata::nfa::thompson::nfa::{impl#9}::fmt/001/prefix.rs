// Answer 0

#[test]
fn test_transition_debug_fmt_single_value_0() {
    let transition = Transition {
        start: 0,
        end: 0,
        next: StateID(SmallIndex(1)),
    };
    let mut output = String::new();
    let _ = fmt::write(&mut output, transition.fmt(&mut fmt::Formatter::new()));
}

#[test]
fn test_transition_debug_fmt_single_value_127() {
    let transition = Transition {
        start: 127,
        end: 127,
        next: StateID(SmallIndex(2)),
    };
    let mut output = String::new();
    let _ = fmt::write(&mut output, transition.fmt(&mut fmt::Formatter::new()));
}

#[test]
fn test_transition_debug_fmt_single_value_255() {
    let transition = Transition {
        start: 255,
        end: 255,
        next: StateID(SmallIndex(3)),
    };
    let mut output = String::new();
    let _ = fmt::write(&mut output, transition.fmt(&mut fmt::Formatter::new()));
}

