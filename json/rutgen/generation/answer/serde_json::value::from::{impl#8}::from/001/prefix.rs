// Answer 0

#[test]
fn test_empty_vec_to_value_array() {
    let v: Vec<Value> = Vec::new();
    let x: Value = v.into();
}

#[test]
fn test_single_element_vec_to_value_array_string() {
    let v = vec![Value::String(String::from("single"))];
    let x: Value = v.into();
}

#[test]
fn test_single_element_vec_to_value_array_number() {
    let v = vec![Value::Number(Number { n: 1 })];
    let x: Value = v.into();
}

#[test]
fn test_single_element_vec_to_value_array_bool() {
    let v = vec![Value::Bool(true)];
    let x: Value = v.into();
}

#[test]
fn test_single_element_vec_to_value_array_null() {
    let v = vec![Value::Null];
    let x: Value = v.into();
}

#[test]
fn test_multiple_elements_vec_to_value_array() {
    let v = vec![
        Value::String(String::from("first")),
        Value::Bool(false),
        Value::Number(Number { n: 10 }),
        Value::Array(vec![Value::String(String::from("nested"))]),
        Value::Object(Map { map: MapImpl::new() }),
    ];
    let x: Value = v.into();
}

#[test]
fn test_large_vec_to_value_array() {
    let v: Vec<Value> = (0..150).map(|i| Value::Number(Number { n: i })).collect();
    let x: Value = v.into();
}

