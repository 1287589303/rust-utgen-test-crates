// Answer 0

#[test]
fn test_serialize_element_null() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = Value::Null;
    let _result = serialize_vec.serialize_element(&value);
}

#[test]
fn test_serialize_element_bool() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = Value::Bool(true);
    let _result = serialize_vec.serialize_element(&value);
}

#[test]
fn test_serialize_element_number() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = Value::Number(Number::from(42));
    let _result = serialize_vec.serialize_element(&value);
}

#[test]
fn test_serialize_element_string() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = Value::String("Hello".to_string());
    let _result = serialize_vec.serialize_element(&value);
}

#[test]
fn test_serialize_element_array() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]);
    let _result = serialize_vec.serialize_element(&value);
}

#[test]
fn test_serialize_element_object() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let mut object = Map::new();
    object.insert("key".to_string(), Value::String("value".to_string()));
    let value = Value::Object(object);
    let _result = serialize_vec.serialize_element(&value);
}

#[test]
fn test_serialize_element_empty_vector() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let value = Value::Array(Vec::new());
    let _result = serialize_vec.serialize_element(&value);
}

#[test]
fn test_serialize_element_multiple_types() {
    let mut serialize_vec = SerializeVec { vec: Vec::new() };
    let mixed_values = vec![
        Value::Null,
        Value::Bool(false),
        Value::Number(Number::from(3.14)),
        Value::String("Rust".to_string()),
        Value::Array(vec![
            Value::String("inner1".to_string()),
            Value::String("inner2".to_string()),
        ]),
        Value::Object(Map::new()),
    ];
    
    for value in mixed_values {
        let _result = serialize_vec.serialize_element(&value);
    }
}

