// Answer 0

#[test]
fn test_read_label_slice_too_short() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 0]; // NUL at the end but size is too short
    let expected_label = "label";
    let result = read_label(slice, expected_label);
}

#[test]
fn test_read_label_slice_exceeds_capacity() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 0, 0]; // NUL found, but slice is too large
    let expected_label = "label";
    let result = read_label(slice, expected_label);
}

#[test]
fn test_read_label_exceeds_capacity_with_padding() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 0, 0, 0, 0]; // NUL at correct place but not enough data for padding
    let expected_label = "label";
    let result = read_label(slice, expected_label);
}

