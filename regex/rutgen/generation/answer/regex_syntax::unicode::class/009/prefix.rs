// Answer 0

#[test]
fn test_class_with_binary_property() {
    let query = ClassQuery::Binary("White_Space");
    let _ = class(query).unwrap();
}

#[test]
fn test_class_with_word_break() {
    let query = ClassQuery::ByValue {
        property_name: "Word_Break",
        property_value: "Any",
    };
    let _ = class(query).unwrap();
}

