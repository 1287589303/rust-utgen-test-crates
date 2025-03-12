// Answer 0

#[test]
fn test_index_object_with_valid_key() {
    let json_object = Value::Object(Map { 
        map: MapImpl::from_iter(vec![("key".to_owned(), Value::String("value".to_owned()))]) 
    });

    let _ = json_object["key"];
}

#[test]
fn test_index_object_with_invalid_key() {
    let json_object = Value::Object(Map { 
        map: MapImpl::from_iter(vec![("key".to_owned(), Value::String("value".to_owned()))]) 
    });

    let _ = json_object["invalid_key"];
}

#[test]
fn test_index_array_with_valid_index() {
    let json_array = Value::Array(vec![Value::String("first".to_owned()), Value::String("second".to_owned())]);

    let _ = json_array[0];
}

#[test]
fn test_index_array_with_invalid_index() {
    let json_array = Value::Array(vec![Value::String("first".to_owned()), Value::String("second".to_owned())]);

    let _ = json_array[2];
}

#[test]
fn test_index_object_with_nested_array() {
    let json_object = Value::Object(Map { 
        map: MapImpl::from_iter(vec![
          ("nested".to_owned(), Value::Array(vec![Value::String("value".to_owned())]))
        ]) 
    });

    let _ = json_object["nested"][0];
}

#[test]
fn test_index_object_with_nonexistent_nested_key() {
    let json_object = Value::Object(Map { 
        map: MapImpl::from_iter(vec![
          ("nested".to_owned(), Value::Array(vec![Value::String("value".to_owned())]))
        ]) 
    });

    let _ = json_object["nested"]["nonexistent"];
}

#[test]
fn test_index_array_with_negative_index() {
    let json_array = Value::Array(vec![Value::String("first".to_owned()), Value::String("second".to_owned())]);

    let _ = json_array[-1];
}

#[test]
fn test_index_of_null_value() {
    let json_object = Value::Object(Map {
        map: MapImpl::from_iter(vec![("key".to_owned(), Value::Null)]) 
    });

    let _ = json_object["key"];
}

