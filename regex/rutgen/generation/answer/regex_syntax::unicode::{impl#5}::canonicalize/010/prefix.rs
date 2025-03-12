// Answer 0

#[test]
fn test_canonicalize_by_value_property_value_not_found() {
    let property_name = "Known_Property"; // Precondition: canonical_prop(&property_name)? is Ok/Some
    let property_value = "Unknown_Value"; // Precondition: canonical_value(vals, &property_value) is None

    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };
    
    let _result = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_property_found() {
    let property_name = "Known_Property"; // Precondition: canonical_prop(&property_name)? is Ok/Some
    let property_value = "Recognized_Value"; // This should be valid but not found in property_values
    
    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };

    let _result = query.canonicalize();
}

