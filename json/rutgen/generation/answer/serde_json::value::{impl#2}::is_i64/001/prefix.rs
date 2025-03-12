// Answer 0

#[test]
fn test_is_i64_with_null() {
    let v = Value::Null;
    v.is_i64();
}

#[test]
fn test_is_i64_with_bool() {
    let v = Value::Bool(true);
    v.is_i64();
}

#[test]
fn test_is_i64_with_string() {
    let v = Value::String(String::from("not a number"));
    v.is_i64();
}

#[test]
fn test_is_i64_with_array() {
    let v = Value::Array(vec![Value::Number(Number::from_i64(10).unwrap()), Value::Number(Number::from_f64(10.5).unwrap())]);
    v.is_i64();
}

#[test]
fn test_is_i64_with_object() {
    let mut obj = Map::new();
    obj.insert(String::from("key"), Value::String(String::from("value")));
    let v = Value::Object(obj);
    v.is_i64();
}

