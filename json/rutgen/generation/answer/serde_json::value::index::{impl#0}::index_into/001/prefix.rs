// Answer 0

#[test]
fn test_index_into_out_of_bounds_negative_index() {
    struct OutOfBoundsIndex(usize);
    
    let index: &OutOfBoundsIndex = &OutOfBoundsIndex(usize::MAX);
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    
    let result = index.index_into(&value);
}

#[test]
fn test_index_into_out_of_bounds_positive_index() {
    struct OutOfBoundsIndex(usize);
    
    let index: &OutOfBoundsIndex = &OutOfBoundsIndex(10);
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    
    let result = index.index_into(&value);
}

#[test]
fn test_index_into_non_array_value() {
    struct NonArrayIndex(usize);
    
    let index: &NonArrayIndex = &NonArrayIndex(0);
    let value = Value::Bool(true); // Not an array
    
    let result = index.index_into(&value);
}

