// Answer 0

#[test]
fn test_serialize_f32_positive_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::INFINITY);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::NEG_INFINITY);
}

#[test]
fn test_serialize_f32_nan() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(f32::NAN);
}

#[test]
fn test_serialize_f32_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(0.0);
}

#[test]
fn test_serialize_f32_negative_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-0.0);
}

#[test]
fn test_serialize_f32_typical_value() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(1.23);
}

#[test]
fn test_serialize_f32_min_value() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-3.4028235e38);
}

#[test]
fn test_serialize_f32_max_value() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(3.4028235e38);
}

#[test]
fn test_serialize_f32_value_just_below_max() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(3.4028235e38 - 1.0e-10);
}

#[test]
fn test_serialize_f32_value_just_above_min() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(-3.4028235e38 + 1.0e-10);
}

