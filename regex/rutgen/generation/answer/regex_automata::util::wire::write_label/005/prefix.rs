// Answer 0

#[test]
fn test_write_label_empty_label() {
    let label = "";
    let nwrite = write_label_len(label);
    let mut dst = vec![0u8; nwrite];
    let _ = write_label(label, &mut dst);
}

#[test]
fn test_write_label_short_label() {
    let label = "test";
    let nwrite = write_label_len(label);
    let mut dst = vec![0u8; nwrite];
    let _ = write_label(label, &mut dst);
}

#[test]
fn test_write_label_max_length_label() {
    let label = "a".repeat(255);
    let nwrite = write_label_len(&label);
    let mut dst = vec![0u8; nwrite];
    let _ = write_label(&label, &mut dst);
}

#[test]
#[should_panic]
fn test_write_label_too_long_label() {
    let label = "a".repeat(256); // this exceeds the max length
    let _ = write_label_len(&label);
}

#[test]
#[should_panic]
fn test_write_label_contains_nul() {
    let label = "test\0label"; // this contains a NUL byte
    let _ = write_label_len(label);
}

