// Answer 0

#[test]
fn test_serialize_bytes_non_empty() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_bytes(&[0, 1, 2]);
}

#[test]
fn test_serialize_bytes_single_byte() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_bytes(&[255]);
}

#[test]
fn test_serialize_bytes_multiple_bytes() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_bytes(&[10, 20, 30, 40, 50]);
}

#[test]
fn test_serialize_bytes_empty() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_bytes(&[]);
}

