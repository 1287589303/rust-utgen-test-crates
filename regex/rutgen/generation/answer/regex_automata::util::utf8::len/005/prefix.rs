// Answer 0

#[test]
fn test_len_boundary_case_fails_first_condition() {
    let byte = 0b1111_1000; // 248
    let result = len(byte);
}

#[test]
fn test_len_boundary_case_valid_return() {
    let byte = 0b1111_0111; // 247
    let result = len(byte);
}

