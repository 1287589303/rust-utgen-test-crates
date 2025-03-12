// Answer 0

#[test]
fn test_write_str_overflow_with_small_bytes() {
    let mut buffer: [u8; 5] = [0; 5];
    let mut buf = Buf { bytes: &mut buffer, offset: 5 };
    let result = buf.write_str("exceeds");
}

#[test]
fn test_write_str_overflow_with_exact_fit() {
    let mut buffer: [u8; 10] = [0; 10];
    let mut buf = Buf { bytes: &mut buffer, offset: 8 };
    let result = buf.write_str("12");
}

#[test]
fn test_write_str_overflow_with_fail_boundary() {
    let mut buffer: [u8; 3] = [0; 3];
    let mut buf = Buf { bytes: &mut buffer, offset: 2 };
    let result = buf.write_str("abc");
}

#[test]
fn test_write_str_overflow_large_string() {
    let mut buffer: [u8; 20] = [0; 20];
    let mut buf = Buf { bytes: &mut buffer, offset: 15 };
    let result = buf.write_str("longerthanallowed");
}

