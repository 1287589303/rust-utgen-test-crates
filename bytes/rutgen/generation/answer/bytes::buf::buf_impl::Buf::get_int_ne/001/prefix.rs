// Answer 0

#[test]
fn test_get_int_ne_1_byte() {
    let mut buf: &[u8] = b"\x7F"; // 127 in signed 1-byte integer
    let _ = buf.get_int_ne(1);
}

#[test]
fn test_get_int_ne_2_bytes() {
    let mut buf: &[u8] = b"\xFF\xFE"; // -2 in signed 2-byte integer
    let _ = buf.get_int_ne(2);
}

#[test]
fn test_get_int_ne_3_bytes_big_endian() {
    let mut buf: &[u8] = b"\x01\x02\x03"; // 66051 in signed 3-byte integer (big-endian)
    let _ = buf.get_int_ne(3);
}

#[test]
fn test_get_int_ne_3_bytes_little_endian() {
    let mut buf: &[u8] = b"\x03\x02\x01"; // 66051 in signed 3-byte integer (little-endian)
    let _ = buf.get_int_ne(3);
}

#[test]
fn test_get_int_ne_4_bytes() {
    let mut buf: &[u8] = b"\xFF\xFF\xFF\xFE"; // -2 in signed 4-byte integer
    let _ = buf.get_int_ne(4);
}

#[test]
fn test_get_int_ne_5_bytes_big_endian() {
    let mut buf: &[u8] = b"\x00\x00\x00\x01\x02"; // 258 in signed 5-byte integer (big-endian)
    let _ = buf.get_int_ne(5);
}

#[test]
fn test_get_int_ne_5_bytes_little_endian() {
    let mut buf: &[u8] = b"\x02\x01\x00\x00\x00"; // 258 in signed 5-byte integer (little-endian)
    let _ = buf.get_int_ne(5);
}

#[test]
fn test_get_int_ne_6_bytes() {
    let mut buf: &[u8] = b"\xFF\xFF\xFF\xFE\x01\x02"; // -2 in signed 6-byte integer
    let _ = buf.get_int_ne(6);
}

#[test]
fn test_get_int_ne_7_bytes() {
    let mut buf: &[u8] = b"\xFF\xFF\xFF\xFF\xFF\xFF\xFE"; // -2 in signed 7-byte integer
    let _ = buf.get_int_ne(7);
}

#[test]
#[should_panic]
fn test_get_int_ne_8_bytes_panics() {
    let mut buf: &[u8] = b"\x00\x00\x00\x00\x00\x00\x00\x00"; // 0 in signed 8-byte integer (but buffer is too small)
    let _ = buf.get_int_ne(8);
}

#[test]
#[should_panic]
fn test_get_int_ne_not_enough_data() {
    let mut buf: &[u8] = b"\x01"; // only 1 byte available
    let _ = buf.get_int_ne(2); // will panic since 2 bytes are requested
}

