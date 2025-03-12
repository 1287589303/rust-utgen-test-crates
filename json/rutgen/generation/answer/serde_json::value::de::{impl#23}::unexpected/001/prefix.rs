// Answer 0

#[test]
fn test_unexpected_object_non_empty() {
    let obj = Value::Object(Map {
        map: vec![("key1".to_string(), Value::Null)].into_iter().collect(),
    });
    let result = obj.unexpected();
}

#[test]
fn test_unexpected_object_single_entry() {
    let obj = Value::Object(Map {
        map: vec![("single_key".to_string(), Value::Bool(true))].into_iter().collect(),
    });
    let result = obj.unexpected();
}

#[test]
fn test_unexpected_object_multiple_entries() {
    let obj = Value::Object(Map {
        map: vec![
            ("key1".to_string(), Value::Number(Number { n: 1 })),
            ("key2".to_string(), Value::String("value".to_string())),
        ].into_iter().collect(),
    });
    let result = obj.unexpected();
}

#[test]
fn test_unexpected_object_empty_string_key() {
    let obj = Value::Object(Map {
        map: vec![("".to_string(), Value::Array(vec![]))].into_iter().collect(),
    });
    let result = obj.unexpected();
}

