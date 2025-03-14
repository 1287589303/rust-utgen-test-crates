// Answer 0

#[test]
fn test_read_u64_valid() {
    let data: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 1]; // Represents 1 in big-endian
    let result = read_u64(data);
    assert_eq!(result, 1);
}

#[test]
fn test_read_u64_boundary() {
    let data: &[u8] = &[255, 255, 255, 255, 255, 255, 255, 255]; // Represents 2^64 - 1
    let result = read_u64(data);
    assert_eq!(result, u64::MAX);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_read_u64_too_short() {
    let data: &[u8] = &[0, 0, 0, 0]; // Only 4 bytes, should panic
    let _result = read_u64(data);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_read_u64_empty() {
    let data: &[u8] = &[]; // No bytes, should panic
    let _result = read_u64(data);
}

