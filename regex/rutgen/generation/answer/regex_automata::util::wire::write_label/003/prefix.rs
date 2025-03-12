// Answer 0

#[test]
fn test_write_label_success() {
    let label = "test";
    let mut dst = vec![0u8; write_label_len(label)];
    let _ = write_label(label, &mut dst);
}

#[test]
fn test_write_label_boundary_length() {
    let label = "a"; // Minimum valid label
    let mut dst = vec![0u8; write_label_len(label)];
    let _ = write_label(label, &mut dst);
}

#[test]
fn test_write_label_exceeding_length() {
    let label = "a".repeat(255); // Maximum valid label length
    let mut dst = vec![0u8; write_label_len(label)];
    let _ = write_label(label, &mut dst);
}

#[test]
#[should_panic]
fn test_write_label_too_long() {
    let label = "a".repeat(256); // One byte over maximum length
    let mut dst = vec![0u8; 0]; // Buffer of size 0
    let _ = write_label(label, &mut dst);
}

#[test]
#[should_panic]
fn test_write_label_with_nul_byte() {
    let label = "test\0"; // Contains a NUL byte
    let mut dst = vec![0u8; write_label_len(label)];
    let _ = write_label(label, &mut dst);
}

#[test]
fn test_write_label_padding() {
    let label = "pad"; // Length is 3
    let expected_length = write_label_len(label);
    let mut dst = vec![0u8; expected_length];
    let _ = write_label(label, &mut dst);
    assert!(dst[(label.len() as usize)..].iter().all(|&x| x == 0));
}

