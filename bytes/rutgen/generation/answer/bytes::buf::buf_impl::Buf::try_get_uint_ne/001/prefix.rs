// Answer 0

#[test]
fn test_try_get_uint_ne_success_big_endian() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04\x05\x06\x07\x08";
    let _ = buf.try_get_uint_ne(3);
}

#[test]
fn test_try_get_uint_ne_success_little_endian() {
    let mut buf: &[u8] = b"\x08\x07\x06\x05\x04\x03\x02\x01";
    let _ = buf.try_get_uint_ne(3);
}

#[test]
fn test_try_get_uint_ne_failure_buffer_too_small() {
    let mut buf: &[u8] = b"\x01\x02\x03";
    let _ = buf.try_get_uint_ne(4);
}

#[test]
#[should_panic]
fn test_try_get_uint_ne_panic_nbytes_greater_than_8() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04\x05\x06\x07\x08";
    let _ = buf.try_get_uint_ne(9);
} 

#[test]
fn test_try_get_uint_ne_exactly_8_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04\x05\x06\x07\x08";
    let _ = buf.try_get_uint_ne(8);
} 

#[test]
fn test_try_get_uint_ne_remaining_bytes() {
    let mut buf: &[u8] = b"\x00\x01\x02\x03\x04";
    let _ = buf.try_get_uint_ne(3);
}


