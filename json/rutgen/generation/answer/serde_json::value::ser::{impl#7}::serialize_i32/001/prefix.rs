// Answer 0

#[test]
fn test_serialize_i32_min() {
    let serializer = MapKeySerializer;
    let value: i32 = -2147483648;
    let _result = serializer.serialize_i32(value);
}

#[test]
fn test_serialize_i32_negative() {
    let serializer = MapKeySerializer;
    let value: i32 = -1;
    let _result = serializer.serialize_i32(value);
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = MapKeySerializer;
    let value: i32 = 0;
    let _result = serializer.serialize_i32(value);
}

#[test]
fn test_serialize_i32_positive() {
    let serializer = MapKeySerializer;
    let value: i32 = 1;
    let _result = serializer.serialize_i32(value);
}

#[test]
fn test_serialize_i32_max() {
    let serializer = MapKeySerializer;
    let value: i32 = 2147483647;
    let _result = serializer.serialize_i32(value);
}

