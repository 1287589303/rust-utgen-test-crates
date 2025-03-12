// Answer 0

#[test]
fn test_string_eq_with_equal_value() {
    let string_value = String::from("equal string");
    let other_value = Value::String(String::from("equal string"));
    string_value.eq(&other_value);
}

#[test]
fn test_string_eq_with_not_equal_value() {
    let string_value = String::from("not equal");
    let other_value = Value::String(String::from("different string"));
    string_value.eq(&other_value);
}

#[test]
fn test_string_eq_with_special_characters() {
    let string_value = String::from("special!@#$%^&*()");
    let other_value = Value::String(String::from("special!@#$%^&*()"));
    string_value.eq(&other_value);
}

#[test]
fn test_string_eq_with_whitespace() {
    let string_value = String::from(" leading and trailing ");
    let other_value = Value::String(String::from(" leading and trailing "));
    string_value.eq(&other_value);
}

#[test]
fn test_string_eq_with_empty_value() {
    let string_value = String::from("non-empty");
    let other_value = Value::String(String::from(""));
    string_value.eq(&other_value);
}

#[test]
fn test_string_eq_with_non_utf8_value() {
    let string_value = String::from("valid utf-8");
    let other_value = Value::String(String::from_utf8_lossy(&[0xff]).to_string());
    string_value.eq(&other_value);
}

