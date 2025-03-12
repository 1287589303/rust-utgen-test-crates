// Answer 0

#[test]
fn test_serialize_i64_min() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i64(i64::MIN);
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_positive() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i64(123456789);
}

#[test]
fn test_serialize_i64_max() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i64(i64::MAX);
}

