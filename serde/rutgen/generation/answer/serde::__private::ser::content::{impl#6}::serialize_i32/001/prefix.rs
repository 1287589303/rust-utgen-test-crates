// Answer 0

#[test]
fn test_serialize_i32_min() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(-2_147_483_648);
}

#[test]
fn test_serialize_i32_negative() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(-1);
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_positive() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(1);
}

#[test]
fn test_serialize_i32_max() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_i32(2_147_483_647);
}

