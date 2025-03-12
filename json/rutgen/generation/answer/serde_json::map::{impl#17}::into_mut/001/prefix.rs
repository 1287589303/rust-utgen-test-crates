// Answer 0

#[test]
fn test_into_mut_with_array_value() {
    let mut map = serde_json::Map::new();
    map.insert("array_key".to_owned(), serde_json::Value::Array(vec![serde_json::Value::Number(serde_json::Number::from(1)), serde_json::Value::Number(serde_json::Number::from(2))]));

    match map.entry("array_key") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            occupied.into_mut().as_array_mut().unwrap().push(serde_json::Value::Number(serde_json::Number::from(3)));
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_into_mut_with_object_value() {
    let mut map = serde_json::Map::new();
    let mut inner_object = serde_json::Map::new();
    inner_object.insert("inner_key".to_owned(), serde_json::Value::String("inner_value".to_owned()));
    
    map.insert("object_key".to_owned(), serde_json::Value::Object(inner_object));

    match map.entry("object_key") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            // This particular case doesn't modify the object but tests the structure
            let obj = occupied.into_mut().as_object_mut().unwrap();
            obj.insert("new_inner_key".to_owned(), serde_json::Value::Bool(true));
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_into_mut_with_empty_array_value() {
    let mut map = serde_json::Map::new();
    map.insert("empty_array_key".to_owned(), serde_json::Value::Array(vec![]));

    match map.entry("empty_array_key") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            occupied.into_mut().as_array_mut().unwrap().push(serde_json::Value::Number(serde_json::Number::from(100)));
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_into_mut_with_multitype_array_value() {
    let mut map = serde_json::Map::new();
    map.insert("multi_type_key".to_owned(), serde_json::Value::Array(vec![
        serde_json::Value::Number(serde_json::Number::from(10)),
        serde_json::Value::String("ten".to_owned()),
        serde_json::Value::Bool(true),
    ]));

    match map.entry("multi_type_key") {
        serde_json::map::Entry::Occupied(mut occupied) => {
            occupied.into_mut().as_array_mut().unwrap().push(serde_json::Value::Null);
        }
        serde_json::map::Entry::Vacant(_) => unimplemented!(),
    }
}

