// Answer 0

#[test]
fn test_serialize_f64_negative_one() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(-1.0);
}

#[test]
fn test_serialize_f64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(0.0);
}

#[test]
fn test_serialize_f64_one() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(1.0);
}

#[test]
fn test_serialize_f64_max() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::MAX);
}

#[test]
fn test_serialize_f64_min() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::MIN);
}

#[test]
fn test_serialize_f64_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_neg_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::NEG_INFINITY);
}

#[test]
fn test_serialize_f64_nan() {
    let serializer = Serializer;
    let result = serializer.serialize_f64(f64::NAN);
}

