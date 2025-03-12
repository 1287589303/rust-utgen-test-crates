// Answer 0

#[test]
fn test_serialize_bytes_empty_array() {
    let writer = Vec::new();
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter: CompactFormatter } };
    let result = serializer.serialize_bytes(&[]);
}

#[test]
fn test_serialize_bytes_single_byte() {
    let writer = Vec::new();
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter: CompactFormatter } };
    let result = serializer.serialize_bytes(&[1]);
}

#[test]
fn test_serialize_bytes_multiple_bytes() {
    let writer = Vec::new();
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter: CompactFormatter } };
    let result = serializer.serialize_bytes(&[1, 2, 3, 4, 5]);
}

