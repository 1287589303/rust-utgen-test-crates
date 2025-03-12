// Answer 0

#[test]
fn test_serialize_bool_false() {
    let serializer = MapKeySerializer;
    let value = false;
    let result = serializer.serialize_bool(value);
}

#[test]
fn test_serialize_bool_true() {
    let serializer = MapKeySerializer;
    let value = true;
    let result = serializer.serialize_bool(value);
}

