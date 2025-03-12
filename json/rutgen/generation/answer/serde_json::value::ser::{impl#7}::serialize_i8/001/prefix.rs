// Answer 0

#[test]
fn test_serialize_i8_negative() {
    let serializer = MapKeySerializer;
    let value = -128i8;
    let _result = serializer.serialize_i8(value);
}

#[test]
fn test_serialize_i8_zero() {
    let serializer = MapKeySerializer;
    let value = 0i8;
    let _result = serializer.serialize_i8(value);
}

#[test]
fn test_serialize_i8_positive() {
    let serializer = MapKeySerializer;
    let value = 127i8;
    let _result = serializer.serialize_i8(value);
}

