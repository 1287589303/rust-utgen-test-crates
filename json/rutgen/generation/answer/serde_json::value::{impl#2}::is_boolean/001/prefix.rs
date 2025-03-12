// Answer 0

#[test]
fn test_is_boolean_true() {
    let v = Value::Bool(true);
    v.is_boolean();
}

#[test]
fn test_is_boolean_false() {
    let v = Value::Bool(false);
    v.is_boolean();
}

#[test]
fn test_is_boolean_null() {
    let v = Value::Null;
    v.is_boolean();
}

#[test]
fn test_is_boolean_string_true() {
    let v = Value::String(String::from("true"));
    v.is_boolean();
}

#[test]
fn test_is_boolean_string_false() {
    let v = Value::String(String::from("false"));
    v.is_boolean();
}

#[test]
fn test_is_boolean_number() {
    let v = Value::Number(Number { n: N });
    v.is_boolean();
}

#[test]
fn test_is_boolean_array() {
    let v = Value::Array(vec![Value::Bool(true)]);
    v.is_boolean();
}

#[test]
fn test_is_boolean_empty_array() {
    let v = Value::Array(vec![]);
    v.is_boolean();
}

#[test]
fn test_is_boolean_object() {
    let v = Value::Object(Map::new());
    v.is_boolean();
}

#[test]
fn test_is_boolean_empty_object() {
    let v = Value::Object(Map::new());
    v.is_boolean();
}

