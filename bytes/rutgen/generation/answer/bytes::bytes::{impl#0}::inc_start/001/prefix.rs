// Answer 0

#[test]
fn test_inc_start_with_self_len_equal_to_by() {
    let mut bytes = Bytes::from_static(&[1, 2, 3, 4]);
    let by = bytes.len(); // self.len == by
    unsafe { bytes.inc_start(by) };
}

#[test]
fn test_inc_start_with_self_len_greater_than_by() {
    let mut bytes = Bytes::from_static(&[1, 2, 3, 4]);
    let by = 2; // self.len > by
    unsafe { bytes.inc_start(by) };
}

