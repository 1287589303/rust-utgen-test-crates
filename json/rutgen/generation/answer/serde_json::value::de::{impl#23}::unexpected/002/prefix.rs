// Answer 0

#[test]
fn test_unexpected_with_empty_array() {
    let value = Value::Array(Vec::new());
    let _result = value.unexpected();
}

#[test]
fn test_unexpected_with_single_element_array() {
    let value = Value::Array(vec![Value::Bool(true)]);
    let _result = value.unexpected();
}

#[test]
fn test_unexpected_with_multi_element_array() {
    let value = Value::Array(vec![Value::Null, Value::String("test".to_owned()), Value::Number(Number { n: 0 })]);
    let _result = value.unexpected();
}

