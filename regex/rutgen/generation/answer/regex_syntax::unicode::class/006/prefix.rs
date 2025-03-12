// Answer 0

#[test]
fn test_class_binary_decimal_number() {
    let query = ClassQuery::Binary("Decimal_Number");
    let _ = class(query);
}

#[test]
fn test_class_binary_white_space() {
    let query = ClassQuery::Binary("White_Space");
    let _ = class(query);
}

#[test]
fn test_class_by_value_script_extensions_valid() {
    let property_value = "Latin";
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value,
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_script_extensions_invalid() {
    let property_value = "InvalidScript";
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value,
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_grapheme_cluster_break_valid() {
    let property_value = "Any";
    let query = ClassQuery::ByValue {
        property_name: "Grapheme_Cluster_Break",
        property_value,
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_word_break_valid() {
    let property_value = "Any";
    let query = ClassQuery::ByValue {
        property_name: "Word_Break",
        property_value,
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_sentence_break_valid() {
    let property_value = "Any";
    let query = ClassQuery::ByValue {
        property_name: "Sentence_Break",
        property_value,
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_age_invalid() {
    let property_value = "AnyOtherValue";
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value,
    };
    let _ = class(query);
}

