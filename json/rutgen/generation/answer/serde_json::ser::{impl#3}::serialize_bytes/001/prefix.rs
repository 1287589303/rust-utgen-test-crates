// Answer 0

#[test]
fn test_serialize_bytes_non_empty() {
    let mut buffer = vec![];
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: &[u8] = &[1, 2, 3, 4, 5];
    serializer.serialize_bytes(value).unwrap();
}

#[test]
fn test_serialize_bytes_empty() {
    let mut buffer = vec![];
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: &[u8] = &[];
    serializer.serialize_bytes(value).unwrap();
}

#[test]
fn test_serialize_bytes_max_size() {
    let mut buffer = vec![];
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: &[u8] = &[255; 64];
    serializer.serialize_bytes(value).unwrap();
}

#[test]
fn test_serialize_bytes_valid_utf8() {
    let mut buffer = vec![];
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: &[u8] = b"Hello, world!";
    serializer.serialize_bytes(value).unwrap();
}

#[test]
fn test_serialize_bytes_all_zeroes() {
    let mut buffer = vec![];
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: &[u8] = &[0; 64];
    serializer.serialize_bytes(value).unwrap();
}

#[test]
fn test_serialize_bytes_special_characters() {
    let mut buffer = vec![];
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: &[u8] = b"Special characters: \n \t \\ \" ";
    serializer.serialize_bytes(value).unwrap();
}

#[test]
fn test_serialize_bytes_edge_case_max_length() {
    let mut buffer = vec![];
    let mut serializer = Serializer { writer: &mut buffer, formatter: CompactFormatter };
    let value: &[u8] = &[128; 64]; // Using a mid-range byte value
    serializer.serialize_bytes(value).unwrap();
}

