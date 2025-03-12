// Answer 0

#[test]
fn test_class_range_invalid() {
    let error = ErrorKind::ClassRangeInvalid;
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| error.fmt(f));
}

#[test]
fn test_class_range_invalid_with_out_of_order_bounds() {
    let error = ErrorKind::ClassRangeInvalid;
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| error.fmt(f));
}

