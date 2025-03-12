// Answer 0

#[test]
fn test_canonical_binary_case_lowercase_mapping() {
    let class_query = ClassQuery::Binary("test");
    let result = class_query.canonical_binary("lc");
}

#[test]
fn test_canonical_binary_case_other_general_category() {
    let class_query = ClassQuery::Binary("test");
    let result = class_query.canonical_binary("assigned");
}

#[test]
fn test_canonical_binary_case_any_general_category() {
    let class_query = ClassQuery::Binary("test");
    let result = class_query.canonical_binary("any");
}

