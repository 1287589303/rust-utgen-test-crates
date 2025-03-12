// Answer 0

#[test]
fn test_write_to_ryu_buffer_zero() {
    let mut buffer: [u8; 16] = [0; 16];
    let result = unsafe { (0.0f32).write_to_ryu_buffer(buffer.as_mut_ptr()) };
    let expected = b"0.0\0";
    assert_eq!(&buffer[..result], expected);
}

#[test]
fn test_write_to_ryu_buffer_infinity() {
    let mut buffer: [u8; 16] = [0; 16];
    let result = unsafe { (f32::INFINITY).write_to_ryu_buffer(buffer.as_mut_ptr()) };
    let expected = b"inf\0";
    assert_eq!(&buffer[..result], expected);
}

#[test]
fn test_write_to_ryu_buffer_neg_infinity() {
    let mut buffer: [u8; 16] = [0; 16];
    let result = unsafe { (-f32::INFINITY).write_to_ryu_buffer(buffer.as_mut_ptr()) };
    let expected = b"-inf\0";
    assert_eq!(&buffer[..result], expected);
}

#[test]
fn test_write_to_ryu_buffer_nan() {
    let mut buffer: [u8; 16] = [0; 16];
    let result = unsafe { f32::NAN.write_to_ryu_buffer(buffer.as_mut_ptr()) };
    let expected = b"NaN\0";
    assert_eq!(&buffer[..result], expected);
}

#[test]
fn test_write_to_ryu_buffer_negative_zero() {
    let mut buffer: [u8; 16] = [0; 16];
    let result = unsafe { (-0.0f32).write_to_ryu_buffer(buffer.as_mut_ptr()) };
    let expected = b"0.0\0";
    assert_eq!(&buffer[..result], expected);
}

