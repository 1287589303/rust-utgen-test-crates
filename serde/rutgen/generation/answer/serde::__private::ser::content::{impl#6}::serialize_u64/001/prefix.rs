// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(0);
}

#[test]
fn test_serialize_u64_min() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(1);
}

#[test]
fn test_serialize_u64_max() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(u64::MAX);
}

#[test]
fn test_serialize_u64_mid() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(123456789);
}

#[test]
fn test_serialize_u64_large_value() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_u64(9876543210);
}

