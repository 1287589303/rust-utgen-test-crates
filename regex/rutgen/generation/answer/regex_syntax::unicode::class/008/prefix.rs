// Answer 0

#[test]
fn test_class_query_binary() {
    let query = ClassQuery::Binary("Decimal_Number");
    let _ = class(query);
}

#[test]
fn test_class_query_by_value_sentence_break() {
    let query = ClassQuery::ByValue {
        property_name: "Sentence_Break",
        property_value: "Any",
    };
    let _ = class(query);
}

