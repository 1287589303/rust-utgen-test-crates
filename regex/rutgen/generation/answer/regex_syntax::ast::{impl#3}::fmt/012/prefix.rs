// Answer 0

#[test]
fn test_fmt_group_unopened() {
    let error_kind = ErrorKind::GroupUnopened;
    let mut output = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut output);
}

#[test]
fn test_fmt_flag_duplicate() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let error_kind = ErrorKind::FlagDuplicate { original: span };
    let mut output = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut output);
}

#[test]
fn test_fmt_group_name_duplicate() {
    let span = Span { start: Position::new(2), end: Position::new(3) };
    let error_kind = ErrorKind::GroupNameDuplicate { original: span };
    let mut output = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut output);
}

