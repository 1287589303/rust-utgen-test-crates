// Answer 0

#[test]
fn test_serialize_f64_min() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(-1.7976931348623157E+308);
}

#[test]
fn test_serialize_f64_max() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(1.7976931348623157E+308);
}

#[test]
fn test_serialize_f64_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(0.0);
}

#[test]
fn test_serialize_f64_neg_zero() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(-0.0);
}

#[test]
fn test_serialize_f64_one() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(1.0);
}

#[test]
fn test_serialize_f64_neg_one() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(-1.0);
}

#[test]
fn test_serialize_f64_nan() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::NAN);
}

#[test]
fn test_serialize_f64_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_negative_infinity() {
    let serializer = ContentSerializer::<()>::default();
    let _ = serializer.serialize_f64(f64::NEG_INFINITY);
}

