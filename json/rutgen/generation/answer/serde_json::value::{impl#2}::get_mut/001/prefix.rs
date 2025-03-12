// Answer 0

#[test]
fn test_get_mut_object_valid_key() {
    let mut object = Value::Object(Map::new());
    object.as_object_mut().unwrap().insert("A".to_string(), Value::Number(Number { n: 1 }));
    let value = object.get_mut("A");
    let result = value.map(|val| {
        *val = Value::Number(Number { n: 2 });
        val
    });
    drop(result);
}

#[test]
fn test_get_mut_object_invalid_key() {
    let mut object = Value::Object(Map::new());
    object.as_object_mut().unwrap().insert("A".to_string(), Value::Number(Number { n: 1 }));
    let value = object.get_mut("B");
    assert!(value.is_none());
}

#[test]
fn test_get_mut_array_valid_index() {
    let mut array = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    let value = array.get_mut(1);
    let result = value.map(|val| {
        *val = Value::Number(Number { n: 3 });
        val
    });
    drop(result);
}

#[test]
fn test_get_mut_array_out_of_bounds_index() {
    let mut array = Value::Array(vec![Value::Number(Number { n: 1 })]);
    let value = array.get_mut(1);
    assert!(value.is_none());
}

#[test]
fn test_get_mut_array_negative_index() {
    let mut array = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    let value = array.get_mut(-1 as usize);
    assert!(value.is_none());
}

#[test]
fn test_get_mut_object_key_does_not_exist() {
    let mut object = Value::Object(Map::new());
    let value = object.get_mut("nonexistent_key");
    assert!(value.is_none());
}

