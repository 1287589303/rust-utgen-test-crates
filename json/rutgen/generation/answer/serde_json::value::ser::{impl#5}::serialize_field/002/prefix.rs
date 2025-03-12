// Answer 0

#[test]
fn test_serialize_field_with_bool() {
    let mut variant = SerializeTupleVariant {
        name: String::from("testVariant"),
        vec: Vec::new(),
    };
    let value = true;
    let _ = variant.serialize_field(&value);
}

#[test]
fn test_serialize_field_with_number() {
    let mut variant = SerializeTupleVariant {
        name: String::from("testVariant"),
        vec: Vec::new(),
    };
    let value = 42;
    let _ = variant.serialize_field(&value);
}

#[test]
fn test_serialize_field_with_string() {
    let mut variant = SerializeTupleVariant {
        name: String::from("testVariant"),
        vec: Vec::new(),
    };
    let value = "a string";
    let _ = variant.serialize_field(&value);
}

#[test]
fn test_serialize_field_with_array() {
    let mut variant = SerializeTupleVariant {
        name: String::from("testVariant"),
        vec: Vec::new(),
    };
    let value = vec![1, 2, 3];
    let _ = variant.serialize_field(&value);
}

#[test]
fn test_serialize_field_with_object() {
    let mut variant = SerializeTupleVariant {
        name: String::from("testVariant"),
        vec: Vec::new(),
    };
    let value = [("key".to_string(), Value::Number(10.into()))].iter().cloned().collect::<Map<String, Value>>();
    let _ = variant.serialize_field(&value);
}

