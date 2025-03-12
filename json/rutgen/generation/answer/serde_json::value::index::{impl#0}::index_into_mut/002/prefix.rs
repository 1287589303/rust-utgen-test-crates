// Answer 0

#[test]
fn test_index_into_mut_with_valid_index() {
    let index = 0;
    let mut value_array = Value::Array(vec![Value::Number(1.0.into()), Value::Number(2.0.into())]);
    let result = (&index).index_into_mut(&mut value_array);
}

#[test]
fn test_index_into_mut_with_last_index() {
    let index = 1;
    let mut value_array = Value::Array(vec![Value::Number(1.0.into()), Value::Number(2.0.into())]);
    let result = (&index).index_into_mut(&mut value_array);
}

#[test]
fn test_index_into_mut_with_out_of_bounds_index() {
    let index = 2;
    let mut value_array = Value::Array(vec![Value::Number(1.0.into()), Value::Number(2.0.into())]);
    let result = (&index).index_into_mut(&mut value_array);
}

