// Answer 0

#[test]
fn test_class_binary_decimal_number() {
    let query = ClassQuery::Binary("Decimal_Number");
    let _ = class(query);
}

#[test]
fn test_class_by_value_grapheme_cluster_break() {
    let query = ClassQuery::ByValue {
        property_name: "Grapheme_Cluster_Break",
        property_value: "Break_Any",
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_word_break() {
    let query = ClassQuery::ByValue {
        property_name: "Word_Break",
        property_value: "Break_Symbols",
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_sentence_break() {
    let query = ClassQuery::ByValue {
        property_name: "Sentence_Break",
        property_value: "Break_After",
    };
    let _ = class(query);
}

