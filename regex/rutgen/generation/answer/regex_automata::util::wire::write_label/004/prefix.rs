// Answer 0

#[test]
fn test_write_label_empty() {
    let label = "";
    let expected_length = write_label_len(label);
    let mut dst = vec![0; expected_length];
    let result = write_label(label, &mut dst);
}

#[test]
fn test_write_label_max_length() {
    let label = "a".repeat(255);
    let expected_length = write_label_len(label.as_str());
    let mut dst = vec![0; expected_length];
    let result = write_label(label.as_str(), &mut dst);
}

#[test]
fn test_write_label_with_padding() {
    let label = "test";
    let expected_length = write_label_len(label);
    let mut dst = vec![0; expected_length];
    let result = write_label(label, &mut dst);
}

#[test]
fn test_write_label_non_null() {
    let label = "hello world";
    let expected_length = write_label_len(label);
    let mut dst = vec![0; expected_length];
    let result = write_label(label, &mut dst);
}

#[test]
fn test_write_label_two_byte_padding() {
    let label = "abc";
    let expected_length = write_label_len(label);
    let mut dst = vec![0; expected_length];
    let result = write_label(label, &mut dst);
}

