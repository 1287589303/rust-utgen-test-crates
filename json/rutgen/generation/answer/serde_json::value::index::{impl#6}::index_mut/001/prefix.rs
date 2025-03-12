// Answer 0

#[test]
fn test_index_mut_insert_into_object() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value["key"] = Value::Number(Number { n: 10 });
}

#[test]
fn test_index_mut_replace_into_object() {
    let mut value = Value::Object(Map {
        map: MapImpl::from_iter(vec![("key".to_string(), Value::Number(Number { n: 5 }))]),
    });
    value["key"] = Value::Number(Number { n: 20 });
}

#[test]
fn test_index_mut_insert_into_array() {
    let mut value = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    value[1] = Value::Number(Number { n: 3 });
}

#[test]
fn test_index_mut_panic_on_index_out_of_bounds() {
    let mut value = Value::Array(vec![Value::Number(Number { n: 1 })]);
    let _ = std::panic::catch_unwind(|| {
        value[2] = Value::Number(Number { n: 4 });
    });
}

#[test]
fn test_index_mut_insert_into_null() {
    let mut value = Value::Null;
    value["new_key"] = Value::Bool(true);
}

#[test]
fn test_index_mut_nested_insert() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value["outer"]["inner"] = Value::String("deep_value".to_string());
}

#[test]
fn test_index_mut_panic_on_invalid_index_type() {
    let mut value = Value::Number(Number { n: 42 });
    let _ = std::panic::catch_unwind(|| {
        value["key"] = Value::Bool(false);
    });
}

