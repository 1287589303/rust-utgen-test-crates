// Answer 0

#[test]
fn test_fmt_with_valid_data() {
    struct TestState<'a> {
        id: StateID,
        stride2: usize,
        transitions: &'a [StateID],
    }

    let transitions: Vec<StateID> = vec![StateID(SmallIndex(1))];
    let state = TestState {
        id: StateID(SmallIndex(0)),
        stride2: 0,
        transitions: &transitions,
    };

    let mut buf = Vec::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    // Simulate alternate being false for the test
    formatter.set_alternate(false);

    state.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_with_start_equal_end() {
    struct TestState<'a> {
        id: StateID,
        stride2: usize,
        transitions: &'a [StateID],
    }

    let transitions: Vec<StateID> = vec![StateID(SmallIndex(1))];
    let state = TestState {
        id: StateID(SmallIndex(0)),
        stride2: 0,
        transitions: &transitions,
    };

    let mut buf = Vec::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    formatter.set_alternate(false);

    state.fmt(&mut formatter).unwrap();
}

