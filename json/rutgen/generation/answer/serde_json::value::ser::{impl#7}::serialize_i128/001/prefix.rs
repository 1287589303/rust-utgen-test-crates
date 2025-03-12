// Answer 0

#[test]
fn test_serialize_i128_min() {
    let serializer = MapKeySerializer;
    let value: i128 = -170141183460469231731687303715884105728;
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_negative() {
    let serializer = MapKeySerializer;
    let value: i128 = -1;
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_zero() {
    let serializer = MapKeySerializer;
    let value: i128 = 0;
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_positive() {
    let serializer = MapKeySerializer;
    let value: i128 = 1;
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_max() {
    let serializer = MapKeySerializer;
    let value: i128 = 170141183460469231731687303715884105727;
    let _result = serializer.serialize_i128(value);
}

