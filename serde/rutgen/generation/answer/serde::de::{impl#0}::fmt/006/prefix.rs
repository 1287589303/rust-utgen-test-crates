// Answer 0

#[test]
fn test_unexpected_enum_variant() {
    let unexpected_enum = Unexpected::Enum;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected_enum.fmt(&mut formatter);
}

#[test]
fn test_unexpected_enum_variant_with_other_expected() {
    let unexpected_enum = Unexpected::Enum;
    let unexpected_other = Unexpected::Signed(10);
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected_enum.fmt(&mut formatter);
    let _ = unexpected_other.fmt(&mut formatter);
}

