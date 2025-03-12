// Answer 0

#[test]
fn test_put_f64_le_with_min_capacity() {
    let mut buf: Vec<u8> = vec![0; 8]; // exactly 8 bytes
    buf.put_f64_le(1.2f64);
}

#[test]
fn test_put_f64_le_with_more_capacity() {
    let mut buf: Vec<u8> = vec![0; 16]; // more than 8 bytes
    buf.put_f64_le(1.2f64);
}

#[test]
fn test_put_f64_le_with_zero() {
    let mut buf: Vec<u8> = vec![0; 8]; // exactly 8 bytes
    buf.put_f64_le(0.0f64);
}

#[test]
fn test_put_f64_le_with_negative() {
    let mut buf: Vec<u8> = vec![0; 8]; // exactly 8 bytes
    buf.put_f64_le(-1.2f64);
}

#[test]
fn test_put_f64_le_with_large_value() {
    let mut buf: Vec<u8> = vec![0; 8]; // exactly 8 bytes
    buf.put_f64_le(1e100f64);
}

#[test]
#[should_panic]
fn test_put_f64_le_with_insufficient_capacity() {
    let mut buf: Vec<u8> = vec![0; 7]; // less than 8 bytes
    buf.put_f64_le(1.2f64);
}

