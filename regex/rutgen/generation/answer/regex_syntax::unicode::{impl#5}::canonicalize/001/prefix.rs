// Answer 0

#[test]
fn test_canonicalize_by_value_property_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "NonCanonicalProperty",
        property_value: "SomeValue",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_property_not_found_empty_value() {
    let query = ClassQuery::ByValue {
        property_name: "AnotherNonCanonicalProperty",
        property_value: "",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_property_not_found_special_chars() {
    let query = ClassQuery::ByValue {
        property_name: "Invalid@PropertyName!",
        property_value: "SomeUniqueValue",
    };
    let _ = query.canonicalize();
}

