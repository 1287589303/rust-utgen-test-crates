// Answer 0

#[test]
fn test_class_one_letter() {
    let query = ClassQuery::OneLetter('A');
    let _result = class(query);
}

#[test]
fn test_class_binary_decimal_number() {
    let query = ClassQuery::Binary("Decimal_Number");
    let _result = class(query);
}

#[test]
fn test_class_binary_white_space() {
    let query = ClassQuery::Binary("White_Space");
    let _result = class(query);
}

#[test]
fn test_class_by_value_age_v1_1() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_script_extensions_arabic() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Arabic",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_grapheme_cluster_break_cr() {
    let query = ClassQuery::ByValue {
        property_name: "Grapheme_Cluster_Break",
        property_value: "CR",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_sentence_break_cr() {
    let query = ClassQuery::ByValue {
        property_name: "Sentence_Break",
        property_value: "CR",
    };
    let _result = class(query);
}

#[test]
fn test_class_by_value_word_break_cr() {
    let query = ClassQuery::ByValue {
        property_name: "Word_Break",
        property_value: "CR",
    };
    let _result = class(query);
}

