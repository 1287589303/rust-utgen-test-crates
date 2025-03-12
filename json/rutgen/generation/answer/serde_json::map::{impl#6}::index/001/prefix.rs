// Answer 0

#[test]
fn test_index_existing_key() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));

    let result = &map["key1"];
}

#[test]
fn test_index_non_existing_key() {
    let mut map = Map {
        map: MapImpl::new(),
    };

    let result = &map["non_existing_key"];
}

#[test]
fn test_index_empty_map() {
    let map = Map {
        map: MapImpl::new(),
    };

    let result = &map["non_existing_key"];
}

