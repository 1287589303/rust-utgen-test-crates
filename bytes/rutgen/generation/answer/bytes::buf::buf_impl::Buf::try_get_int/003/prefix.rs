// Answer 0

#[test]
fn test_try_get_int_success_1_byte() {
    let mut buf = &b"\x01Hello"[..];
    let _ = buf.try_get_int(1);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_success_2_bytes() {
    let mut buf = &b"\x01\x02Hello"[..];
    let _ = buf.try_get_int(2);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_success_3_bytes() {
    let mut buf = &b"\x01\x02\x03Hello"[..];
    let _ = buf.try_get_int(3);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_success_4_bytes() {
    let mut buf = &b"\x01\x02\x03\x04Hello"[..];
    let _ = buf.try_get_int(4);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_success_5_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05Hello"[..];
    let _ = buf.try_get_int(5);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_success_6_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06Hello"[..];
    let _ = buf.try_get_int(6);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_success_7_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07Hello"[..];
    let _ = buf.try_get_int(7);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_success_8_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08Hello"[..];
    let _ = buf.try_get_int(8);
    let _ = buf.remaining();
}

#[test]
fn test_try_get_int_error_9_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let _ = buf.try_get_int(9);
    let _ = buf.remaining();
}

#[test]
#[should_panic]
fn test_try_get_int_panic_over_8_bytes() {
    let mut buf = &b"\x01\x02\x03"[..];
    let _ = buf.try_get_int(9);
}

