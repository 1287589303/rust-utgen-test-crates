// Answer 0

#[test]
fn test_format_group_name_empty() {
    let error_kind = ErrorKind::GroupNameEmpty;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

#[test]
fn test_format_group_name_duplicate() {
    let error_kind = ErrorKind::GroupNameDuplicate {
        original: Span { start: Position::new(0), end: Position::new(1) }
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

