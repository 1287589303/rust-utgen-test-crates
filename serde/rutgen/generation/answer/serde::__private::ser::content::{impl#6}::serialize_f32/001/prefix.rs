// Answer 0

#[test]
fn test_serialize_f32_negative_boundary() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(-3.4028235e38);
}

#[test]
fn test_serialize_f32_negative_one() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(-1.0);
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(0.0);
}

#[test]
fn test_serialize_f32_one() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(1.0);
}

#[test]
fn test_serialize_f32_positive_boundary() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(3.4028235e38);
}

#[test]
fn test_serialize_f32_nan() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(f32::NAN);
}

#[test]
fn test_serialize_f32_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_f32(f32::NEG_INFINITY);
}

