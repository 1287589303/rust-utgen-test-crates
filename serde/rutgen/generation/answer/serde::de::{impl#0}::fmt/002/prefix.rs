// Answer 0

#[test]
fn test_display_struct_variant() {
    let unexpected_variant = Unexpected::StructVariant;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected_variant.fmt(&mut formatter);
}

#[test]
fn test_display_with_other_variant_as_struct_variant() {
    let unexpected_variant = Unexpected::Other("unexpected data");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected_variant.fmt(&mut formatter);
}

