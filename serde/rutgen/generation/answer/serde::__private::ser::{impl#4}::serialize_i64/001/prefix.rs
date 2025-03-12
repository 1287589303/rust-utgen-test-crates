// Answer 0

#[test]
fn test_serialize_i64_min() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i64(i64::MIN);
}

#[test]
fn test_serialize_i64_zero() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_max() {
    let mut map = HashMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i64(i64::MAX);
}

