// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_max() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u8(255);
}

#[test]
fn test_serialize_u8_mid() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u8(128);
}

#[test]
fn test_serialize_u8_one() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_fifty() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_u8(50);
}

