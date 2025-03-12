// Answer 0

#[test]
fn test_as_u64_null() {
    let value = Value::Null;
    value.as_u64();
}

#[test]
fn test_as_u64_bool() {
    let value = Value::Bool(true);
    value.as_u64();
}

#[test]
fn test_as_u64_string() {
    let value = Value::String(String::from("test"));
    value.as_u64();
}

#[test]
fn test_as_u64_array() {
    let value = Value::Array(vec![]);
    value.as_u64();
}

#[test]
fn test_as_u64_object() {
    let value = Value::Object(Map::new());
    value.as_u64();
}

