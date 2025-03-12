// Answer 0

#[test]
fn test_index_into_mut_with_negative_index() {
    struct ArrayIndex(usize);
    impl Index for ArrayIndex {
        // Implementation omitted 
    }
    
    let index = ArrayIndex(usize::MAX); // Out of bounds
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let result = index.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_index_greater_than_length() {
    struct ArrayIndex(usize);
    impl Index for ArrayIndex {
        // Implementation omitted 
    }
    
    let index = ArrayIndex(2); // Out of bounds for an array of length 2
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let result = index.index_into_mut(&mut value);
}

