// Answer 0

#[test]
fn test_try_get_int_success_1_byte() {
    let mut buf = &mut [0x01u8; 8][..];
    buf.try_get_int(1).unwrap();
}

#[test]
fn test_try_get_int_success_2_bytes() {
    let mut buf = &mut [0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..];
    assert_eq!(Ok(0x00000201_i64), buf.try_get_int(2));
}

#[test]
fn test_try_get_int_success_3_bytes() {
    let mut buf = &mut [0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00][..];
    assert_eq!(Ok(0x0000030201_i64), buf.try_get_int(3));
}

#[test]
fn test_try_get_int_success_4_bytes() {
    let mut buf = &mut [0x01, 0x02, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00][..];
    assert_eq!(Ok(0x00000004030201_i64), buf.try_get_int(4));
}

#[test]
fn test_try_get_int_success_5_bytes() {
    let mut buf = &mut [0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00][..];
    assert_eq!(Ok(0x0000000504030201_i64), buf.try_get_int(5));
}

#[test]
fn test_try_get_int_success_6_bytes() {
    let mut buf = &mut [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00, 0x00][..];
    assert_eq!(Ok(0x000000060504030201_i64), buf.try_get_int(6));
}

#[test]
fn test_try_get_int_success_7_bytes() {
    let mut buf = &mut [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00][..];
    assert_eq!(Ok(0x00000007060504030201_i64), buf.try_get_int(7));
}

#[test]
#[should_panic]
fn test_try_get_int_panic_too_large() {
    let mut buf = &mut [0u8; 8][..];
    buf.try_get_int(9);
}

#[test]
fn test_try_get_int_error_not_enough_bytes() {
    let mut buf = &mut [0x01, 0x02][..];
    assert_eq!(Err(TryGetError { requested: 3, available: 2 }), buf.try_get_int(3));
}

