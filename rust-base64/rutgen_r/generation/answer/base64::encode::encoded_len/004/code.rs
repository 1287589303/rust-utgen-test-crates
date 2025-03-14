// Answer 0

#[test]
fn test_encoded_len_with_padding_zero() {
    let bytes_len = 1; 
    let padding = true; 
    assert_eq!(base64::encoded_len(bytes_len, padding), Some(4));
}

#[test]
fn test_encoded_len_without_padding() {
    let bytes_len = 1; 
    let padding = false; 
    assert_eq!(base64::encoded_len(bytes_len, padding), Some(2));
} 

#[test]
fn test_encoded_len_with_non_zero_padding() {
    let bytes_len = 4; 
    let padding = true; 
    assert_eq!(base64::encoded_len(bytes_len, padding), Some(8));
}

#[test]
fn test_encoded_len_without_padding_for_two() {
    let bytes_len = 2; 
    let padding = false; 
    assert_eq!(base64::encoded_len(bytes_len, padding), Some(3));
}

