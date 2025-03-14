// Answer 0

#[test]
fn test_encoded_len_with_rem_1_and_no_padding() {
    let bytes_len = 1; // This will produce a remainder of 1 when divided by 3
    let padding = false; // We set padding to false
    let result = base64::encoded_len(bytes_len, padding);
    assert_eq!(result, Some(2)); // Given rem == 1, we expect the length to be 2
}

#[test]
fn test_encoded_len_with_rem_2_and_no_padding() {
    let bytes_len = 2; // This will produce a remainder of 2 when divided by 3
    let padding = false; // We set padding to false
    let result = base64::encoded_len(bytes_len, padding);
    assert_eq!(result, Some(3)); // Given rem == 2, we expect the length to be 3
}

#[test]
fn test_encoded_len_with_complete_chunks_and_rem_1_no_padding() {
    let bytes_len = 4; // This will produce complete chunks (1 full chunk of 3 and remainder of 1)
    let padding = false; // We set padding to false
    let result = base64::encoded_len(bytes_len, padding);
    assert_eq!(result, Some(6)); // 1 complete chunk (4) + 2 extra for the remainder
}

#[test]
fn test_encoded_len_with_complete_chunks_and_rem_2_no_padding() {
    let bytes_len = 5; // This will produce complete chunks (1 full chunk of 3 and remainder of 2)
    let padding = false; // We set padding to false
    let result = base64::encoded_len(bytes_len, padding);
    assert_eq!(result, Some(8)); // 1 complete chunk (4) + 3 extra for the remainder
}

