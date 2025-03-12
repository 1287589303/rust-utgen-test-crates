// Answer 0

#[test]
fn test_serialize_u8_zero() {
    struct DummySerializer;
    let serializer = DummySerializer;
    let result = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_mid() {
    struct DummySerializer;
    let serializer = DummySerializer;
    let result = serializer.serialize_u8(128);
}

#[test]
fn test_serialize_u8_max() {
    struct DummySerializer;
    let serializer = DummySerializer;
    let result = serializer.serialize_u8(255);
}

