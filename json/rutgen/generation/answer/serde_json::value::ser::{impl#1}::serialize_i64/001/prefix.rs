// Answer 0

#[test]
fn test_serialize_i64_min() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(-9223372036854775808);
}

#[test]
fn test_serialize_i64_max() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(9223372036854775807);
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_negative_one() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(-1);
}

#[test]
fn test_serialize_i64_one() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(1);
}

#[test]
fn test_serialize_i64_negative_max() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(-9223372036854775807);
}

#[test]
fn test_serialize_i64_positive_max() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(9223372036854775806);
}

