// Answer 0

#[test]
fn test_class_binary_property_valid() {
    let query = ClassQuery::Binary("Decimal_Number");
    let _result = class(query);
}

#[test]
fn test_class_script_property_valid() {
    let query = ClassQuery::Script("Latin");
    let _result = class(query);
}

#[test]
fn test_class_by_value_age_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_script_extensions_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latin",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_grapheme_cluster_break_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Grapheme_Cluster_Break",
        property_value: "CR",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_sentence_break_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Sentence_Break",
        property_value: "Other",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_word_break_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Word_Break",
        property_value: "AL",
    };
    let _result = class(query);
}

