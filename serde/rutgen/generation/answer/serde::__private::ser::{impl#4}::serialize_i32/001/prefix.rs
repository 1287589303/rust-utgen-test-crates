// Answer 0

#[test]
fn test_serialize_i32_min() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i32(i32::MIN);
}

#[test]
fn test_serialize_i32_negative_one() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i32(-1);
}

#[test]
fn test_serialize_i32_zero() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_one() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i32(1);
}

#[test]
fn test_serialize_i32_max() {
    let mut map = std::collections::HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i32(i32::MAX);
}

