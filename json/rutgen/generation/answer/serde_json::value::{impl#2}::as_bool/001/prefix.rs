// Answer 0

#[test]
fn test_as_bool_null() {
    let v = Value::Null;
    v.as_bool();
}

#[test]
fn test_as_bool_number() {
    let v = Value::Number(Number { n: 0 });
    v.as_bool();
}

#[test]
fn test_as_bool_string() {
    let v = Value::String("example".to_string());
    v.as_bool();
}

#[test]
fn test_as_bool_array() {
    let v = Value::Array(Vec::new());
    v.as_bool();
}

#[test]
fn test_as_bool_object() {
    let v = Value::Object(Map { map: MapImpl::new() });
    v.as_bool();
}

