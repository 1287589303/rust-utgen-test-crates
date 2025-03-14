// Answer 0

#[test]
fn test_new_with_zero_bytes_and_none_padding() {
    let decoded_bytes = 0;
    let padding_index = None;

    let result = base64::new(decoded_bytes, padding_index);

    assert_eq!(result.decoded_len, 0);
    assert_eq!(result.padding_offset, None);
}

#[test]
fn test_new_with_non_zero_bytes_and_none_padding() {
    let decoded_bytes = 10;
    let padding_index = None;

    let result = base64::new(decoded_bytes, padding_index);

    assert_eq!(result.decoded_len, 10);
    assert_eq!(result.padding_offset, None);
}

#[test]
fn test_new_with_zero_bytes_and_some_padding() {
    let decoded_bytes = 0;
    let padding_index = Some(2);

    let result = base64::new(decoded_bytes, padding_index);

    assert_eq!(result.decoded_len, 0);
    assert_eq!(result.padding_offset, Some(2));
}

#[test]
fn test_new_with_non_zero_bytes_and_some_padding() {
    let decoded_bytes = 15;
    let padding_index = Some(3);

    let result = base64::new(decoded_bytes, padding_index);

    assert_eq!(result.decoded_len, 15);
    assert_eq!(result.padding_offset, Some(3));
}

