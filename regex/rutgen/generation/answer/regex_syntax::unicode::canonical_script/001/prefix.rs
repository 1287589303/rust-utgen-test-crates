// Answer 0

#[test]
fn test_canonical_script_property_not_found() {
    let normalized_value = "some_script_value";
    let result = canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_empty_string() {
    let normalized_value = "";
    let result = canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_invalid_value() {
    let normalized_value = "invalid_script";
    let result = canonical_script(normalized_value);
}

#[test]
fn test_canonical_script_another_invalid_value() {
    let normalized_value = "totally_not_a_script";
    let result = canonical_script(normalized_value);
}

