// Answer 0

#[test]
fn test_transition_fmt_range() {
    let transition = Transition {
        start: 100,
        end: 200,
        next: StateID(SmallIndex(5)),
    };
    let mut buf = String::new();
    let f = &mut fmt::Formatter::for_str(&mut buf);
    let _ = transition.fmt(f);
}

#[test]
fn test_transition_fmt_boundary() {
    let transition = Transition {
        start: 254,
        end: 255,
        next: StateID(SmallIndex(3)),
    };
    let mut buf = String::new();
    let f = &mut fmt::Formatter::for_str(&mut buf);
    let _ = transition.fmt(f);
}

#[test]
fn test_transition_fmt_minimal_range() {
    let transition = Transition {
        start: 0,
        end: 1,
        next: StateID(SmallIndex(0)),
    };
    let mut buf = String::new();
    let f = &mut fmt::Formatter::for_str(&mut buf);
    let _ = transition.fmt(f);
}

#[test]
fn test_transition_fmt_larger_range() {
    let transition = Transition {
        start: 1,
        end: 10,
        next: StateID(SmallIndex(2)),
    };
    let mut buf = String::new();
    let f = &mut fmt::Formatter::for_str(&mut buf);
    let _ = transition.fmt(f);
}

