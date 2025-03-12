// Answer 0

#[test]
fn test_property_values_valid_short_name() {
    let result = property_values("L");
}

#[test]
fn test_property_values_valid_long_name() {
    let result = property_values("unicode-general-category");
}

#[test]
fn test_property_values_empty_string() {
    let result = property_values("");
}

#[test]
fn test_property_values_invalid_name() {
    let result = property_values("invalid-property-name");
}

#[test]
fn test_property_values_valid_name_with_boundary() {
    let result = property_values("Cc");
}

