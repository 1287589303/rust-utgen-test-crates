// Answer 0

#[test]
fn test_fmt_start_state_iter_valid() {
    struct DummyStartTable;
    let dummy_table = DummyStartTable;

    let formatter = &mut fmt::Formatter::default();
    let state_iter = StartStateIter {
        st: &dummy_table,
        i: 0,
    };

    let _ = state_iter.fmt(formatter);
}

#[test]
fn test_fmt_start_state_iter_boundary() {
    struct DummyStartTable;
    let dummy_table = DummyStartTable;

    let formatter = &mut fmt::Formatter::default();
    let state_iter = StartStateIter {
        st: &dummy_table,
        i: usize::MAX,
    };

    let _ = state_iter.fmt(formatter);
}

