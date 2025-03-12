// Answer 0

#[test]
fn test_eq_str_with_empty_string() {
    let value = Value::String(String::new());
    let other = "";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_non_empty_string() {
    let value = Value::String(String::from("Hello, world!"));
    let other = "Hello, world!";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_special_characters() {
    let value = Value::String(String::from("!@#$%^&*()"));
    let other = "!@#$%^&*()";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_null_value() {
    let value = Value::Null;
    let other = "null";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_boolean_true() {
    let value = Value::Bool(true);
    let other = "true";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_boolean_false() {
    let value = Value::Bool(false);
    let other = "false";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_number() {
    let value = Value::Number(Number::from(42));
    let other = "42";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_array() {
    let value = Value::Array(vec![Value::String(String::from("item1")), Value::String(String::from("item2"))]);
    let other = "item1";
    eq_str(&value, other);
}

#[test]
fn test_eq_str_with_object() {
    let mut map = std::collections::BTreeMap::new();
    map.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(map);
    let other = "key";
    eq_str(&value, other);
}

