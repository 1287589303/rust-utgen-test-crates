// Answer 0

#[test]
fn test_canonical_binary_property() {
    let class_query = ClassQuery::Binary("SomeBinaryProperty");
    let result = class_query.canonical_binary("SomeBinaryProperty");
}

#[test]
fn test_canonical_binary_general_category() {
    let class_query = ClassQuery::Binary("SomeGeneralCategory");
    let result = class_query.canonical_binary("SomeGeneralCategory");
}

#[test]
fn test_canonical_binary_script() {
    let class_query = ClassQuery::Binary("SomeScript");
    let result = class_query.canonical_binary("SomeScript");
}

