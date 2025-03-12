// Answer 0

#[test]
fn test_class_range_literal() {
    use core::fmt::Formatter;
    let error_kind = regex_syntax::ErrorKind::ClassRangeLiteral;
    let mut formatter = Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

#[test]
fn test_class_range_literal_with_internal_logic() {
    use core::fmt::Formatter;
    let error_kind = regex_syntax::ErrorKind::ClassRangeLiteral;
    let mut formatter = Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

