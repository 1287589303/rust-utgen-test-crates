// Answer 0

#[test]
fn test_retain_empty_map() {
    let mut map = Map::new();
    map.retain(|_k, _v| false);
}

#[test]
fn test_retain_various_values() {
    let mut map = Map::new();
    map.insert("bool".to_string(), Value::Bool(true));
    map.insert("number".to_string(), Value::Number(42.0.into()));
    map.insert("string".to_string(), Value::String("example".to_string()));
    map.insert("array".to_string(), Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]));
    map.insert("object".to_string(), Value::Object(Map::new()));

    map.retain(|k, v| {
        if k == "bool" {
            true
        } else if let Value::Number(_) = v {
            true
        } else {
            false
        }
    });
}

#[test]
fn test_retain_all_removed() {
    let mut map = Map::new();
    map.insert("example".to_string(), Value::Number(1.0.into()));
    map.insert("test".to_string(), Value::String("test".to_string()));

    map.retain(|_k, _v| false);
}

#[test]
fn test_retain_specific_keys() {
    let mut map = Map::new();
    map.insert("keep".to_string(), Value::Number(1.0.into()));
    map.insert("remove".to_string(), Value::String("test".to_string()));

    map.retain(|k, _v| k == "keep");
}

#[test]
fn test_retain_large_map() {
    let mut map = Map::with_capacity(10_000);
    for i in 0..10_000 {
        map.insert(format!("key_{}", i), Value::Number(i.into()));
    }
    map.retain(|k, _v| k.contains('_'));
}

#[test]
fn test_retain_special_character_keys() {
    let mut map = Map::new();
    map.insert("key_with_space".to_string(), Value::String("value1".to_string()));
    map.insert("key$special!@#".to_string(), Value::String("value2".to_string()));

    map.retain(|k, _v| k.contains('$'));
}

