// Answer 0

#[test]
fn test_error_empty_range_display() {
    let error_variant = Error::EmptyRange;
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let _ = error_variant.fmt(formatter);
}

#[test]
#[should_panic]
fn test_error_empty_range_empty_case() {
    let error_variant = Error::EmptyRange;
    let result = error_variant.fmt(&mut fmt::Formatter::new(&mut String::new()));
}

