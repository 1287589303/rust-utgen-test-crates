// Answer 0

#[test]
fn test_decode_metadata_new_with_padding() {
    let decoded_bytes = 5;
    let padding_index = Some(2);
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, decoded_bytes);
    assert_eq!(metadata.padding_offset, padding_index);
}

#[test]
fn test_decode_metadata_new_without_padding() {
    let decoded_bytes = 8;
    let padding_index = None;
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, decoded_bytes);
    assert_eq!(metadata.padding_offset, padding_index);
}

#[test]
fn test_decode_metadata_equality() {
    let metadata1 = DecodeMetadata::new(3, Some(1));
    let metadata2 = DecodeMetadata::new(3, Some(1));
    let metadata3 = DecodeMetadata::new(4, None);
    assert_eq!(metadata1, metadata2);
    assert_ne!(metadata1, metadata3);
}

