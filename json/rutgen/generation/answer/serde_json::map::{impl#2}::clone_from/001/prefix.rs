// Answer 0

#[test]
fn test_clone_from_non_empty() {
    let mut map1 = Map { 
        map: BTreeMap::new() 
    };
    let mut map2 = Map { 
        map: BTreeMap::new() 
    };
    
    map1.map.insert("key1".to_string(), Value::Bool(true));
    map1.map.insert("key2".to_string(), Value::Number(Number::from(3.14)));

    map2.clone_from(&map1);
}

#[test]
fn test_clone_from_with_existing_data() {
    let mut map1 = Map { 
        map: BTreeMap::new() 
    };
    let mut map2 = Map { 
        map: BTreeMap::new() 
    };
    
    map1.map.insert("key1".to_string(), Value::String("value1".to_string()));
    map1.map.insert("key2".to_string(), Value::Null);

    map2.map.insert("key3".to_string(), Value::Number(Number::from(42)));

    map2.clone_from(&map1);
}

#[test]
fn test_clone_from_with_multiple_entries() {
    let mut map1 = Map { 
        map: BTreeMap::new() 
    };
    let mut map2 = Map { 
        map: BTreeMap::new() 
    };

    map1.map.insert("keyA".to_string(), Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]));
    map1.map.insert("keyB".to_string(), Value::Object(Map { map: BTreeMap::new() }));

    map2.clone_from(&map1);
}

