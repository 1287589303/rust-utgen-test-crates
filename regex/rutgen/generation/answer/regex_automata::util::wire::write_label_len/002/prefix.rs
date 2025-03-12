// Answer 0

#[test]
#[should_panic(expected = "label must not be longer than 255 bytes")]
fn test_write_label_len_too_long_label() {
    let label = "a".repeat(256); // label length exceeds 255
    let _ = write_label_len(&label);
}

#[test]
#[should_panic(expected = "label must not contain NUL bytes")]
fn test_write_label_len_label_with_nul() {
    let label = "label\0with_nul"; // label contains NUL byte
    let _ = write_label_len(&label);
}

#[test]
fn test_write_label_len_boundary_case_255_bytes() {
    let label = "a".repeat(255); // label length exactly 255
    let _ = write_label_len(&label);
}

#[test]
fn test_write_label_len_boundary_case_0_bytes() {
    let label = ""; // label length is 0
    let _ = write_label_len(&label);
}

#[test]
fn test_write_label_len_boundary_case_1_byte() {
    let label = "a"; // label length is 1
    let _ = write_label_len(&label);
}

#[test]
fn test_write_label_len_boundary_case_mid_length() {
    let label = "12345678901234567890"; // label length is 20
    let _ = write_label_len(&label);
}

#[test]
#[should_panic(expected = "label must not contain NUL bytes")]
fn test_write_label_len_label_with_nul_at_start() {
    let label = "\0label"; // NUL byte at start
    let _ = write_label_len(&label);
}

#[test]
#[should_panic(expected = "label must not contain NUL bytes")]
fn test_write_label_len_label_with_nul_at_end() {
    let label = "label\0"; // NUL byte at end
    let _ = write_label_len(&label);
}

#[test]
#[should_panic(expected = "label must not contain NUL bytes")]
fn test_write_label_len_label_with_nul_in_middle() {
    let label = "label\0middle"; // NUL byte in middle
    let _ = write_label_len(&label);
}

