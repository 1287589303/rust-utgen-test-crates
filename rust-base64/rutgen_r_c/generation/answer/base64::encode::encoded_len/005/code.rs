// Answer 0

#[test]
fn test_encoded_len_zero_bytes_no_padding() {
    let bytes_len = 0;
    let padding = false;
    assert_eq!(encoded_len(bytes_len, padding), Some(0));
}

#[test]
fn test_encoded_len_three_bytes_no_padding() {
    let bytes_len = 3;
    let padding = false;
    assert_eq!(encoded_len(bytes_len, padding), Some(4));
}

#[test]
fn test_encoded_len_multiple_of_three_with_padding() {
    let bytes_len = 6;
    let padding = true;
    assert_eq!(encoded_len(bytes_len, padding), Some(8));
}

#[test]
fn test_encoded_len_multiple_of_three_without_padding() {
    let bytes_len = 9;
    let padding = false;
    assert_eq!(encoded_len(bytes_len, padding), Some(12));
}

