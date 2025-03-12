// Answer 0

#[test]
fn test_canonical_binary_case_format() {
    let query = ClassQuery::Binary("cf");
    let result = query.canonical_binary("cf");
}

#[test]
fn test_canonical_binary_case_any() {
    let query = ClassQuery::ByValue { property_name: "General_Category", property_value: "any" };
    let result = query.canonical_binary("any");
}

#[test]
fn test_canonical_binary_case_assigned() {
    let query = ClassQuery::ByValue { property_name: "General_Category", property_value: "assigned" };
    let result = query.canonical_binary("assigned");
}

