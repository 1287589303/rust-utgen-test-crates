// Answer 0

#[test]
fn test_serialize_i64_negative_edge_case() {
    let serializer = MapKeySerializer;
    let value: i64 = i64::MIN;
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_negative_one() {
    let serializer = MapKeySerializer;
    let value: i64 = -1;
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = MapKeySerializer;
    let value: i64 = 0;
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_one() {
    let serializer = MapKeySerializer;
    let value: i64 = 1;
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_positive_edge_case() {
    let serializer = MapKeySerializer;
    let value: i64 = i64::MAX;
    let _result = serializer.serialize_i64(value);
}

