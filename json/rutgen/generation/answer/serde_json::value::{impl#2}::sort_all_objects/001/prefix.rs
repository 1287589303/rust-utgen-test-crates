// Answer 0

#[test]
fn test_sort_all_objects_with_preserve_order_enabled() {
    let mut value = Value::Object(Map {
        map: MapImpl::new(),
    });
    
    value.as_object_mut().unwrap().insert("b".to_string(), Value::Object(Map {
        map: MapImpl::new(),
    }));
    
    value.as_object_mut().unwrap().insert("a".to_string(), Value::Object(Map {
        map: MapImpl::new(),
    }));
    
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_nested_array_with_preserve_order_enabled() {
    let mut value = Value::Array(vec![
        Value::Object(Map {
            map: MapImpl::new(),
        }),
        Value::Object(Map {
            map: MapImpl::new(),
        }),
    ]);
    
    if let Value::Array(arr) = value.as_array_mut().unwrap() {
        arr[0] = Value::Object(Map {
            map: MapImpl::new(),
        });
        arr[1] = Value::Object(Map {
            map: MapImpl::new(),
        });
    }
    
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_empty_object() {
    let mut value = Value::Object(Map {
        map: MapImpl::new(),
    });
    
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_empty_array() {
    let mut value = Value::Array(vec![]);
    
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_deeply_nested_structure_with_preserve_order_enabled() {
    let mut value = Value::Object(Map {
        map: MapImpl::new(),
    });

    value.as_object_mut().unwrap().insert("outer".to_string(), Value::Array(vec![
        Value::Object(Map {
            map: MapImpl::new(),
        }),
        Value::Object(Map {
            map: MapImpl::new(),
        }),
    ]));
    
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_with_no_objects() {
    let mut value = Value::Bool(true);
    
    value.sort_all_objects();
}

#[test]
fn test_sort_all_objects_with_integers() {
    let mut value = Value::Array(vec![
        Value::Number(Number { n: 1 }),
        Value::Number(Number { n: 2 }),
    ]);
    
    value.sort_all_objects();
}

