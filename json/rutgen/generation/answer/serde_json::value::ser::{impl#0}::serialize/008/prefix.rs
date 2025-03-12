// Answer 0

#[test]
fn test_serialize_value_bool_true() {
    let value = Value::Bool(true);
    // Assuming we have a serializer in scope
    let serializer = ...; // Initialize your serializer here
    let _ = value.serialize(serializer);
}

#[test]
fn test_serialize_value_bool_false() {
    let value = Value::Bool(false);
    // Assuming we have a serializer in scope
    let serializer = ...; // Initialize your serializer here
    let _ = value.serialize(serializer);
}

