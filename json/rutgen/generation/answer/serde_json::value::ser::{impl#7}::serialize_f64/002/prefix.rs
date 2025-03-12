// Answer 0

#[test]
fn test_serialize_f64_nan() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(f64::NAN);
}

#[test]
fn test_serialize_f64_positive_infinity() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_negative_infinity() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(f64::NEG_INFINITY);
}

