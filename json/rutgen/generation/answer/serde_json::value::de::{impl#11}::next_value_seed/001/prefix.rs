// Answer 0

#[test]
fn test_next_value_seed_bool() {
    let value = Value::Bool(true);
    let mut deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: Some(value),
    };
    let seed = serde_json::de::from_str::<bool>("true").unwrap();
    deserializer.next_value_seed(seed).unwrap();
}

#[test]
fn test_next_value_seed_number() {
    let value = Value::Number(Number::from(12.5));
    let mut deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: Some(value),
    };
    let seed = serde_json::de::from_str::<f64>("12.5").unwrap();
    deserializer.next_value_seed(seed).unwrap();
}

#[test]
fn test_next_value_seed_string() {
    let value = Value::String(String::from("test string"));
    let mut deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: Some(value),
    };
    let seed = serde_json::de::from_str::<String>(r#""test string""#).unwrap();
    deserializer.next_value_seed(seed).unwrap();
}

#[test]
fn test_next_value_seed_array() {
    let value = Value::Array(vec![Value::Bool(false), Value::Number(Number::from(1))]);
    let mut deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: Some(value),
    };
    let seed = serde_json::de::from_str::<Vec<Value>>(r#"[false, 1]"#).unwrap();
    deserializer.next_value_seed(seed).unwrap();
}

#[test]
fn test_next_value_seed_object() {
    let value = Value::Object(Map::new());
    let mut deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: Some(value),
    };
    let seed = serde_json::de::from_str::<Map<String, Value>>(r#"{}"#).unwrap();
    deserializer.next_value_seed(seed).unwrap();
}

#[test]
fn test_next_value_seed_none() {
    let mut deserializer = MapDeserializer {
        iter: vec![].into_iter(),
        value: None,
    };
    let seed = serde_json::de::from_str::<bool>("false").unwrap();
    let result = deserializer.next_value_seed(seed);
    assert!(result.is_err());
}

