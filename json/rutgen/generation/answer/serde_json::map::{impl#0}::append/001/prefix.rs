// Answer 0

#[test]
fn test_append_empty_maps() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    map1.append(&mut map2);
}

#[test]
fn test_append_non_empty_to_empty_map() {
    let mut map1 = Map::new();
    let mut map2 = Map::new();
    map2.insert("key1".to_string(), Value::Null);
    map2.insert("key2".to_string(), Value::Bool(true));
    map1.append(&mut map2);
}

#[test]
fn test_append_empty_to_non_empty_map() {
    let mut map1 = Map::new();
    map1.insert("key1".to_string(), Value::Number(Number::from(10)));
    let mut map2 = Map::new();
    map1.append(&mut map2);
}

#[test]
fn test_append_non_empty_maps() {
    let mut map1 = Map::new();
    map1.insert("key1".to_string(), Value::String("value1".to_string()));
    let mut map2 = Map::new();
    map2.insert("key2".to_string(), Value::Array(vec![Value::Number(Number::from(1))]));
    map1.append(&mut map2);
}

#[test]
fn test_append_with_various_value_types() {
    let mut map1 = Map::new();
    map1.insert("key1".to_string(), Value::String("value1".to_string()));
    map1.insert("key2".to_string(), Value::Bool(false));
    
    let mut map2 = Map::new();
    map2.insert("key3".to_string(), Value::Array(vec![Value::Number(Number::from(2)), Value::Number(Number::from(3))]));
    map2.insert("key4".to_string(), Value::Object(Map::new()));
    
    map1.append(&mut map2);
}

#[test]
fn test_append_large_maps() {
    let mut map1 = Map::with_capacity(1000);
    for i in 0..1000 {
        map1.insert(format!("key{}", i), Value::Number(Number::from(i)));
    }
    
    let mut map2 = Map::with_capacity(1000);
    for i in 1000..2000 {
        map2.insert(format!("key{}", i), Value::String(format!("value{}", i)));
    }
    
    map1.append(&mut map2);
}

#[test]
fn test_append_with_preserve_order_feature() {
    #[cfg(feature = "preserve_order")]
    {
        let mut map1 = Map::new();
        map1.insert("first".to_string(), Value::String("First".to_string()));
        
        let mut map2 = Map::new();
        map2.insert("second".to_string(), Value::String("Second".to_string()));
        
        map1.append(&mut map2);
    }
} 

#[test]
fn test_append_maps_with_preserve_order() {
    #[cfg(feature = "preserve_order")]
    {
        let mut map1 = Map::new();
        map1.insert("a".to_string(), Value::Bool(true));
        map1.insert("b".to_string(), Value::Null);
        
        let mut map2 = Map::new();
        map2.insert("c".to_string(), Value::Number(Number::from(3.14)));
        map2.insert("d".to_string(), Value::Array(vec![Value::String("item".to_string())]));
        
        map1.append(&mut map2);
    }
}

