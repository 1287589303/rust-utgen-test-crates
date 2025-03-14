// Answer 0

#[test]
fn test_new_with_zero_decoded_bytes_and_none_padding() {
    let decoded_bytes = 0;
    let padding_index = None;
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata, DecodeMetadata { decoded_len: 0, padding_offset: None });
}

#[test]
fn test_new_with_non_zero_decoded_bytes_and_none_padding() {
    let decoded_bytes = 5;
    let padding_index = None;
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata, DecodeMetadata { decoded_len: 5, padding_offset: None });
}

#[test]
fn test_new_with_zero_decoded_bytes_and_some_padding() {
    let decoded_bytes = 0;
    let padding_index = Some(3);
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata, DecodeMetadata { decoded_len: 0, padding_offset: Some(3) });
}

#[test]
fn test_new_with_non_zero_decoded_bytes_and_some_padding() {
    let decoded_bytes = 10;
    let padding_index = Some(2);
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata, DecodeMetadata { decoded_len: 10, padding_offset: Some(2) });
}

