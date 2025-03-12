// Answer 0

#[test]
fn test_get_u64_available_bytes_zero() {
    let mut buf: &[u8] = &[];
    buf.get_u64();
}

#[test]
#[should_panic]
fn test_get_u64_available_bytes_one() {
    let mut buf: &[u8] = &[0x01];
    buf.get_u64();
}

#[test]
#[should_panic]
fn test_get_u64_available_bytes_two() {
    let mut buf: &[u8] = &[0x01, 0x02];
    buf.get_u64();
}

#[test]
#[should_panic]
fn test_get_u64_available_bytes_three() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03];
    buf.get_u64();
}

#[test]
#[should_panic]
fn test_get_u64_available_bytes_four() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04];
    buf.get_u64();
}

#[test]
#[should_panic]
fn test_get_u64_available_bytes_five() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];
    buf.get_u64();
}

#[test]
#[should_panic]
fn test_get_u64_available_bytes_six() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    buf.get_u64();
}

#[test]
#[should_panic]
fn test_get_u64_available_bytes_seven() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    buf.get_u64();
}

#[test]
fn test_get_u64_available_bytes_eight() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let result = buf.get_u64();
    assert_eq!(result, 0x0102030405060708);
}

#[test]
fn test_get_u64_available_bytes_more_than_eight() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    let result = buf.get_u64();
    assert_eq!(result, 0x0102030405060708);
}

#[test]
fn test_get_u64_non_aligned_bytes() {
    let mut buf: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let result = buf.get_u64();
    assert_eq!(result, 0x0001020304050607);
}

