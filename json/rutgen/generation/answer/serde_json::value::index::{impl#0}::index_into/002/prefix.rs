// Answer 0

#[test]
fn test_index_into_valid_0() {
    let value_array = Value::Array(vec![Value::Bool(true)]);
    let index = 0;
    let result = index_into(&index, &value_array);
}

#[test]
fn test_index_into_valid_1() {
    let value_array = Value::Array(vec![Value::Bool(true), Value::Number(Number::from(10))]);
    let index = 1;
    let result = index_into(&index, &value_array);
}

#[test]
fn test_index_into_bound_limit() {
    let value_array = Value::Array(vec![Value::String(String::from("test"))]);
    let index = 0;
    let result = index_into(&index, &value_array);
}

#[test]
fn test_index_into_out_of_bounds() {
    let value_array = Value::Array(vec![Value::Null]);
    let index = 1;
    let result = index_into(&index, &value_array);
}

