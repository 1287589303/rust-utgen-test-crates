// Answer 0

#[test]
fn test_read_label_success() {
    let expected_label = "A".repeat(255);
    let slice: Vec<u8> = expected_label.as_bytes().iter().cloned().chain(vec![0x00]).collect();
    let result = read_label(&slice, &expected_label);
}

#[test]
fn test_read_label_boundary() {
    let expected_label = "B".repeat(255);
    let slice: Vec<u8> = expected_label.as_bytes().iter().cloned().chain(vec![0x00]).collect();
    let result = read_label(&slice, &expected_label);
}

