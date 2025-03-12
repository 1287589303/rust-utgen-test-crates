// Answer 0

#[test]
fn test_index_into_mut_with_null() {
    let mut value = Value::Null;
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_bool() {
    let mut value = Value::Bool(true);
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_number() {
    let mut value = Value::Number(Number::from(42));
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_string() {
    let mut value = Value::String("test".to_owned());
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_empty_array() {
    let mut value = Value::Array(Vec::new());
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_empty_object() {
    let mut value = Value::Object(Map::new());
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_large_array() {
    let mut value = Value::Array(vec![Value::Number(Number::from(i)) for i in 0..1000]);
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_large_object() {
    let mut value = Value::Object((0..100).map(|i| (i.to_string(), Value::Number(Number::from(i)))).collect());
    let reference: &dyn Index = &value;
    reference.index_into_mut(&mut value);
}

