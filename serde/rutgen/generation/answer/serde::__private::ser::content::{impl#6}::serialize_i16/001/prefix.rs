// Answer 0

#[test]
fn test_serialize_i16_min() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i16(-32768);
}

#[test]
fn test_serialize_i16_middle() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i16(0);
}

#[test]
fn test_serialize_i16_max() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i16(32767);
}

