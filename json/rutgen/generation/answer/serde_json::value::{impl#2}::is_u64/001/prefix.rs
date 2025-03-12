// Answer 0

#[test]
fn test_is_u64_null() {
    let value = Value::Null;
    let _ = value.is_u64();
}

#[test]
fn test_is_u64_bool_false() {
    let value = Value::Bool(false);
    let _ = value.is_u64();
}

#[test]
fn test_is_u64_string() {
    let value = Value::String(String::from("not a number"));
    let _ = value.is_u64();
}

#[test]
fn test_is_u64_array() {
    let value = Value::Array(vec![Value::String(String::from("element"))]);
    let _ = value.is_u64();
}

#[test]
fn test_is_u64_object() {
    let value = Value::Object(Map { map: std::collections::BTreeMap::new() });
    let _ = value.is_u64();
}

