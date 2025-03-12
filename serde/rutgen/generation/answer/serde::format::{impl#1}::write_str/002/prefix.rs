// Answer 0

#[test]
fn test_write_str_exact_fit() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf { bytes: &mut buffer, offset: 5 };
    let s = "hello"; // len(s) = 5
    let result = buf.write_str(s);
}

#[test]
fn test_write_str_zero_length() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf { bytes: &mut buffer, offset: 10 };
    let s = ""; // len(s) = 0
    let result = buf.write_str(s);
}

