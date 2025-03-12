// Answer 0

#[test]
fn test_fmt_with_zero_attempted() {
    let error = LazyStateIDError { attempted: 0 };
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_max_attempted() {
    let error = LazyStateIDError { attempted: LazyStateID::MAX };
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_exceeding_attempted() {
    let error = LazyStateIDError { attempted: LazyStateID::MAX + 1 };
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

