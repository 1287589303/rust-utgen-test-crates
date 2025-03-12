// Answer 0

#[test]
fn test_class_property_not_found() {
    let query = ClassQuery::Binary("Invalid_Property_Name");
    let _ = class(query);
}

#[test]
fn test_class_one_letter_not_found() {
    let query = ClassQuery::OneLetter('∫'); // A character that does not correspond to any known binary property
    let _ = class(query);
}

#[test]
fn test_class_by_value_invalid_property() {
    let query = ClassQuery::ByValue {
        property_name: "Invalid_Property",
        property_value: "Some_Value",
    };
    let _ = class(query);
}

