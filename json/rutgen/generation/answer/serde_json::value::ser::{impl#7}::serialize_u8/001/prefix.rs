// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_min() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_mid() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u8(128);
}

#[test]
fn test_serialize_u8_max() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u8(255);
}

