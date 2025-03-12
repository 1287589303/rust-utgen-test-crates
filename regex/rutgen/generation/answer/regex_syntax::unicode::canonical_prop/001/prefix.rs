// Answer 0

#[test]
fn test_valid_property_name() {
    let normalized_name = "General_Category";
    let result = canonical_prop(normalized_name);
}

#[test]
fn test_non_existing_property_name() {
    let normalized_name = "Non_Existing_Property";
    let result = canonical_prop(normalized_name);
}

#[test]
fn test_empty_property_name() {
    let normalized_name = "";
    let result = canonical_prop(normalized_name);
}

