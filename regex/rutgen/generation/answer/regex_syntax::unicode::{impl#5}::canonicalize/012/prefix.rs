// Answer 0

#[test]
fn test_canonicalize_by_value_invalid_property_name() {
    let query = ClassQuery::ByValue {
        property_name: "invalid_property_name",
        property_value: "some_value",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_invalid_property_value() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "invalid_value",
    };
    let _ = query.canonicalize();
}

