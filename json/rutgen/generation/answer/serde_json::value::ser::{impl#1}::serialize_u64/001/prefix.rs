// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(0);
}

#[test]
fn test_serialize_u64_middle() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(1_000_000);
}

#[test]
fn test_serialize_u64_max() {
    let serializer = Serializer;
    let result = serializer.serialize_u64(u64::MAX);
}

