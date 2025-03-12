// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let serializer = Serializer;
    let value: &[u8] = &[];
    let _ = serializer.serialize_bytes(value);
}

#[test]
fn test_serialize_bytes_single() {
    let serializer = Serializer;
    let value: &[u8] = &[42];
    let _ = serializer.serialize_bytes(value);
}

#[test]
fn test_serialize_bytes_valid() {
    let serializer = Serializer;
    let value: &[u8] = &[0, 255];
    let _ = serializer.serialize_bytes(value);
}

#[test]
fn test_serialize_bytes_maximum() {
    let serializer = Serializer;
    let value: &[u8] = &[u8::MAX];
    let _ = serializer.serialize_bytes(value);
}

