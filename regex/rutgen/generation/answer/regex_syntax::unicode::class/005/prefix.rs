// Answer 0

#[test]
fn test_class_binary() {
    let query = ClassQuery::Binary("White_Space");
    let result = class(query);
}

#[test]
fn test_class_by_value_age_v1_1() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let result = class(query);
}

#[test]
fn test_class_by_value_age_v2_0() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V2_0",
    };
    let result = class(query);
}

#[test]
fn test_class_by_value_grapheme_cluster_break() {
    let query = ClassQuery::ByValue {
        property_name: "Grapheme_Cluster_Break",
        property_value: "Extend",
    };
    let result = class(query);
}

#[test]
fn test_class_by_value_word_break() {
    let query = ClassQuery::ByValue {
        property_name: "Word_Break",
        property_value: "CR",
    };
    let result = class(query);
}

#[test]
fn test_class_by_value_script_extensions() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latin",
    };
    let result = class(query);
}

#[test]
fn test_class_by_value_sentence_break() {
    let query = ClassQuery::ByValue {
        property_name: "Sentence_Break",
        property_value: "Sentence_Terminal",
    };
    let result = class(query);
}

