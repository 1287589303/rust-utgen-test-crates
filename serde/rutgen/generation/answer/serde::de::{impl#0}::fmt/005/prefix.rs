// Answer 0

#[test]
fn test_disp_unit_variant() {
    use crate::Unexpected;

    let unexpected_variant = Unexpected::UnitVariant;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexpected_variant.fmt(&mut formatter);
}

#[test]
fn test_display_for_unit_variant() {
    use crate::Unexpected;

    let unexpected_variant = Unexpected::UnitVariant;
    let mut formatter = std::fmt::Formatter::new();
    let _ = write!(formatter, "{}", unexpected_variant);
}

