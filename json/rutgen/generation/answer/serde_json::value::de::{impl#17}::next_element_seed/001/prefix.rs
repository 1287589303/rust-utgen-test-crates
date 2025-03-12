// Answer 0

#[test]
fn test_next_element_seed_bool() {
    let value = Value::Bool(true);
    let values = vec![value];
    let mut deserializer = SeqRefDeserializer { iter: values.iter() };
    let seed = serde::de::seed::IgnoredAny;
    let _ = deserializer.next_element_seed(seed);
}

#[test]
fn test_next_element_seed_number() {
    let value = Value::Number(Number::from(42));
    let values = vec![value];
    let mut deserializer = SeqRefDeserializer { iter: values.iter() };
    let seed = serde::de::seed::IgnoredAny;
    let _ = deserializer.next_element_seed(seed);
}

#[test]
fn test_next_element_seed_string() {
    let value = Value::String("test".to_string());
    let values = vec![value];
    let mut deserializer = SeqRefDeserializer { iter: values.iter() };
    let seed = serde::de::seed::IgnoredAny;
    let _ = deserializer.next_element_seed(seed);
}

#[test]
fn test_next_element_seed_array() {
    let value = Value::Array(vec![Value::Bool(false)]);
    let values = vec![value];
    let mut deserializer = SeqRefDeserializer { iter: values.iter() };
    let seed = serde::de::seed::IgnoredAny;
    let _ = deserializer.next_element_seed(seed);
}

#[test]
fn test_next_element_seed_object() {
    let value = Value::Object(Map::new());
    let values = vec![value];
    let mut deserializer = SeqRefDeserializer { iter: values.iter() };
    let seed = serde::de::seed::IgnoredAny;
    let _ = deserializer.next_element_seed(seed);
}

#[test]
fn test_next_element_seed_null() {
    let value = Value::Null;
    let values = vec![value];
    let mut deserializer = SeqRefDeserializer { iter: values.iter() };
    let seed = serde::de::seed::IgnoredAny;
    let _ = deserializer.next_element_seed(seed);
}

