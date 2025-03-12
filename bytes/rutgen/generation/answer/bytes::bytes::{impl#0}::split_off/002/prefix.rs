// Answer 0

#[test]
fn test_split_off_middle() {
    let mut bytes = Bytes::copy_from_slice(b"hello world");
    let result = bytes.split_off(5);
}

#[test]
fn test_split_off_non_empty() {
    let mut bytes = Bytes::copy_from_slice(b"abcdefgh");
    let result = bytes.split_off(4);
}

#[test]
fn test_split_off_boundaries() {
    let mut bytes = Bytes::copy_from_slice(b"data");
    let result = bytes.split_off(1);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut bytes = Bytes::copy_from_slice(b"panic");
    let _result = bytes.split_off(5);
}

