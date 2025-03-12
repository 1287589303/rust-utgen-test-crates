// Answer 0

#[test]
fn test_eq_empty_maps() {
    let map1 = Map {
        map: MapImpl::<String, Value>::new(),
    };
    let map2 = Map {
        map: MapImpl::<String, Value>::new(),
    };
    let _ = map1.eq(&map2);
}

#[test]
fn test_eq_different_empty_maps() {
    let map1 = Map {
        map: MapImpl::<String, Value>::new(),
    };
    let map2 = Map {
        map: MapImpl::<String, Value>::new(),
    };
    let _ = map1.eq(&map2);
}

#[test]
fn test_eq_matching_maps() {
    let mut map1_data = MapImpl::<String, Value>::new();
    map1_data.insert("key1".to_string(), Value::Null);
    let map1 = Map { map: map1_data };

    let mut map2_data = MapImpl::<String, Value>::new();
    map2_data.insert("key1".to_string(), Value::Null);
    let map2 = Map { map: map2_data };

    let _ = map1.eq(&map2);
}

#[test]
fn test_eq_different_keys() {
    let mut map1_data = MapImpl::<String, Value>::new();
    map1_data.insert("key1".to_string(), Value::Bool(true));
    let map1 = Map { map: map1_data };

    let mut map2_data = MapImpl::<String, Value>::new();
    map2_data.insert("key2".to_string(), Value::Bool(true));
    let map2 = Map { map: map2_data };

    let _ = map1.eq(&map2);
}

#[test]
fn test_eq_different_values() {
    let mut map1_data = MapImpl::<String, Value>::new();
    map1_data.insert("key1".to_string(), Value::Number(Number::from(42)));
    let map1 = Map { map: map1_data };

    let mut map2_data = MapImpl::<String, Value>::new();
    map2_data.insert("key1".to_string(), Value::Number(Number::from(43)));
    let map2 = Map { map: map2_data };

    let _ = map1.eq(&map2);
}

#[test]
fn test_eq_mixed_value_types() {
    let mut map1_data = MapImpl::<String, Value>::new();
    map1_data.insert("null_key".to_string(), Value::Null);
    map1_data.insert("bool_key".to_string(), Value::Bool(true));
    map1_data.insert("number_key".to_string(), Value::Number(Number::from(42)));
    let map1 = Map { map: map1_data };

    let mut map2_data = MapImpl::<String, Value>::new();
    map2_data.insert("null_key".to_string(), Value::Null);
    map2_data.insert("bool_key".to_string(), Value::Bool(true));
    map2_data.insert("number_key".to_string(), Value::Number(Number::from(42)));
    let map2 = Map { map: map2_data };

    let _ = map1.eq(&map2);
}

