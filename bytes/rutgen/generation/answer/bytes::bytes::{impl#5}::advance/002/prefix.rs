// Answer 0

#[test]
fn test_advance_with_cnt_greater_than_len() {
    let mut bytes = Bytes::new(); // Represents an empty Bytes instance, so len() is 0
    let cnt = 1; // cnt > len(), as len() is 0
    bytes.advance(cnt);
}

#[test]
#[should_panic] // Ensures it panics when cnt is greater than len
fn test_advance_with_cnt_greater_than_len_boundary_case_1() {
    let bytes = Bytes::from_static(b"test"); // len() is 4
    let cnt = 5; // cnt > len(), len() is 4
    bytes.advance(cnt);
}

#[test]
#[should_panic] // Ensures it panics when cnt is greater than len
fn test_advance_with_cnt_greater_than_len_boundary_case_2() {
    let bytes = Bytes::from_static(b"bytes"); // len() is 5
    let cnt = 6; // cnt > len(), len() is 5
    bytes.advance(cnt);
}

#[test]
#[should_panic] // Ensures it panics when cnt is greater than len
fn test_advance_with_cnt_max() {
    let bytes = Bytes::from_static(b"max"); // len() is 3
    let cnt = usize::MAX; // cnt is greater than len(), len() is 3
    bytes.advance(cnt);
}

