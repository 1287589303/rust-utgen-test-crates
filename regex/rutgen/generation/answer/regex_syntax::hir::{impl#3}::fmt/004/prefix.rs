// Answer 0

#[test]
fn test_unicode_property_not_found() {
    let error_kind = crate::ErrorKind::UnicodePropertyNotFound;
    let mut formatter = core::fmt::Formatter::new();
    error_kind.fmt(&mut formatter);
}

#[test]
fn test_unicode_property_value_not_found() {
    let error_kind = crate::ErrorKind::UnicodePropertyValueNotFound;
    let mut formatter = core::fmt::Formatter::new();
    error_kind.fmt(&mut formatter);
}

