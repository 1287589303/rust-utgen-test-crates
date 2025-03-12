// Answer 0

#[test]
fn test_canonicalize_by_value_script_property_value_found() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "Latin",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_script_property_value_none() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "UnknownScript",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_general_category_false_script_true() {
    let query = ClassQuery::ByValue {
        property_name: "Some_Other_Property",
        property_value: "Latin",
    };
    let _ = query.canonicalize();
}

