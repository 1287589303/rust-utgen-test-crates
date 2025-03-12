// Answer 0

#[test]
fn test_class_binary_property() {
    let query = ClassQuery::Binary("White_Space");
    let _result = class(query);
}

#[test]
fn test_class_by_value_age() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_grapheme_cluster_break() {
    let query = ClassQuery::ByValue {
        property_name: "Grapheme_Cluster_Break",
        property_value: "Control",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_word_break() {
    let query = ClassQuery::ByValue {
        property_name: "Word_Break",
        property_value: "Alphabetic",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_script_extensions() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latin",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_sentence_break() {
    let query = ClassQuery::ByValue {
        property_name: "Sentence_Break",
        property_value: "CR",
    };
    let _result = class(query);
}

