// Answer 0

#[test]
fn test_canonicalize_general_category_some() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "assigned"
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_general_category_none() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "invalid_category"
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_general_category_any() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "any"
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_property_name_invalid() {
    let query = ClassQuery::ByValue {
        property_name: "Invalid_Property",
        property_value: "assigned"
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_property_value_empty() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: ""
    };
    let _ = query.canonicalize();
}

