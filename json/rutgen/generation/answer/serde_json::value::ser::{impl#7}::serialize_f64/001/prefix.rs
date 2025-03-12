// Answer 0

#[test]
fn test_serialize_finite_positive_max() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(1.7976931348623157E+308);
}

#[test]
fn test_serialize_finite_positive() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(123.456);
}

#[test]
fn test_serialize_finite_negative_max() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(-1.7976931348623157E+308);
}

#[test]
fn test_serialize_finite_negative() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(-123.456);
}

#[test]
fn test_serialize_finite_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_f64(0.0);
}

