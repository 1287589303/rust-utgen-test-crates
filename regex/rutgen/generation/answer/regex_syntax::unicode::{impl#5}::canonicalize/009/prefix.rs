// Answer 0

#[test]
fn test_canonicalize_by_value_success() {
    let query = ClassQuery::ByValue {
        property_name: "ValidProperty",
        property_value: "ValidValue",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_property_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "InvalidProperty",
        property_value: "ValidValue",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_general_category_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "SomeProperty",
        property_value: "InvalidValue",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_script_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "InvalidValue",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_values_none() {
    let query = ClassQuery::ByValue {
        property_name: "SomeProperty",
        property_value: "AnotherValidValue",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_values_some() {
    let query = ClassQuery::ByValue {
        property_name: "SomeProperty",
        property_value: "AnotherValidValue",
    };
    let result = query.canonicalize();
}

