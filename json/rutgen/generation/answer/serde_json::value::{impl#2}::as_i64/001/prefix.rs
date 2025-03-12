// Answer 0

#[test]
fn test_as_i64_with_null() {
    let v = Value::Null;
    v.as_i64();
}

#[test]
fn test_as_i64_with_bool() {
    let v = Value::Bool(true);
    v.as_i64();
}

#[test]
fn test_as_i64_with_string() {
    let v = Value::String(String::from("test"));
    v.as_i64();
}

#[test]
fn test_as_i64_with_array() {
    let v = Value::Array(vec![Value::Bool(false), Value::String(String::from("array"))]);
    v.as_i64();
}

#[test]
fn test_as_i64_with_object() {
    let v = Value::Object(Map::new());
    v.as_i64();
}

