// Answer 0

#[test]
fn test_serialize_i8_negative_boundary() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i8(-128);
}

#[test]
fn test_serialize_i8_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i8(0);
}

#[test]
fn test_serialize_i8_positive_boundary() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i8(127);
}

#[test]
fn test_serialize_i8_middle_value() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i8(64);
}

#[test]
fn test_serialize_i8_negative_value() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_i8(-64);
}

