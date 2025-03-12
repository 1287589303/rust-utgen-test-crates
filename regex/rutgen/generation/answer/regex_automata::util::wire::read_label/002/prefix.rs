// Answer 0

#[test]
fn test_read_label_label_mismatch() {
    let slice: &[u8] = b"wrong_label\0"; // first_nul is at index 11, len is 12
    let expected_label: &'static str = "correct_label";
    
    let result = read_label(slice, expected_label);
    result.unwrap_err(); // Expecting an error for label mismatch
}

#[test]
fn test_read_label_correct_length_with_mismatch() {
    let slice: &[u8] = b"test_label\0\0\0"; // first_nul at index 10, len is 12
    let expected_label: &'static str = "different_label";
    
    let result = read_label(slice, expected_label);
    result.unwrap_err(); // Expecting an error for label mismatch
}

#[test]
fn test_read_label_edge_case() {
    let slice: &[u8] = b"boundary_label\0\0\0"; // first_nul at index 14, len is 16
    let expected_label: &'static str = "another_label";
    
    let result = read_label(slice, expected_label);
    result.unwrap_err(); // Expecting an error for label mismatch
}

