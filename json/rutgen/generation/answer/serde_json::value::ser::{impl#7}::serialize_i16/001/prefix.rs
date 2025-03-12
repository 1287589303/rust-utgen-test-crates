// Answer 0

#[test]
fn test_serialize_i16_negative_boundary() {
    let serializer = MapKeySerializer;
    let value: i16 = -32_768;
    let result = serializer.serialize_i16(value);
}

#[test]
fn test_serialize_i16_negative_one() {
    let serializer = MapKeySerializer;
    let value: i16 = -1;
    let result = serializer.serialize_i16(value);
}

#[test]
fn test_serialize_i16_zero() {
    let serializer = MapKeySerializer;
    let value: i16 = 0;
    let result = serializer.serialize_i16(value);
}

#[test]
fn test_serialize_i16_one() {
    let serializer = MapKeySerializer;
    let value: i16 = 1;
    let result = serializer.serialize_i16(value);
}

#[test]
fn test_serialize_i16_positive_boundary() {
    let serializer = MapKeySerializer;
    let value: i16 = 32_767;
    let result = serializer.serialize_i16(value);
}

