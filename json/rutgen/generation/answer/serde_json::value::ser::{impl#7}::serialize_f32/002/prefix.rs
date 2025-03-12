// Answer 0

#[test]
fn test_serialize_f32_nan() {
    let serializer = MapKeySerializer;
    let value = f32::NAN;
    let _result = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_f32_infinity() {
    let serializer = MapKeySerializer;
    let value = f32::INFINITY;
    let _result = serializer.serialize_f32(value);
}

#[test]
fn test_serialize_f32_negative_infinity() {
    let serializer = MapKeySerializer;
    let value = f32::NEG_INFINITY;
    let _result = serializer.serialize_f32(value);
}

