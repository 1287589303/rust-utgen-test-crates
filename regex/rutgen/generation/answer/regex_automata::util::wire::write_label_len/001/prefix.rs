// Answer 0

#[test]
#[should_panic]
fn test_write_label_len_too_long() {
    let long_label = "a".repeat(256);
    let _ = write_label_len(&long_label);
}

