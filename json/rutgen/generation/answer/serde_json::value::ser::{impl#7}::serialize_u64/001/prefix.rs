// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let serializer = MapKeySerializer;
    let value: u64 = 0;
    let _result = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_max() {
    let serializer = MapKeySerializer;
    let value: u64 = u64::MAX; // which is 2^64 - 1
    let _result = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_small_positive() {
    let serializer = MapKeySerializer;
    let value: u64 = 1;
    let _result = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_large_positive() {
    let serializer = MapKeySerializer;
    let value: u64 = 123456789;
    let _result = serializer.serialize_u64(value);
}

#[test]
fn test_serialize_u64_mid_range() {
    let serializer = MapKeySerializer;
    let value: u64 = 2_000_000_000;
    let _result = serializer.serialize_u64(value);
}

