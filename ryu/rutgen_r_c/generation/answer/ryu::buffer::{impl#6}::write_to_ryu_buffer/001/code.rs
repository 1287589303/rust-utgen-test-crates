// Answer 0

#[test]
fn test_write_to_ryu_buffer_zero() {
    let mut buffer: [u8; 20] = [0; 20];
    let result = unsafe { (0.0_f32).write_to_ryu_buffer(buffer.as_mut_ptr()) };
    assert_eq!(result, 4); // Length of "0.0"
    assert_eq!(str::from_utf8(&buffer[..result]).unwrap(), "0.0");
}

#[test]
fn test_write_to_ryu_buffer_negative_zero() {
    let mut buffer: [u8; 20] = [0; 20];
    let result = unsafe { (-0.0_f32).write_to_ryu_buffer(buffer.as_mut_ptr()) };
    assert_eq!(result, 4); // Length of "-0.0"
    assert_eq!(str::from_utf8(&buffer[..result]).unwrap(), "-0.0");
}

#[test]
fn test_write_to_ryu_buffer_positive_infinity() {
    let mut buffer: [u8; 20] = [0; 20];
    let result = unsafe { f32::INFINITY.write_to_ryu_buffer(buffer.as_mut_ptr()) };
    assert_eq!(result, 3); // Length of "inf"
    assert_eq!(str::from_utf8(&buffer[..result]).unwrap(), "inf");
}

#[test]
fn test_write_to_ryu_buffer_negative_infinity() {
    let mut buffer: [u8; 20] = [0; 20];
    let result = unsafe { (-f32::INFINITY).write_to_ryu_buffer(buffer.as_mut_ptr()) };
    assert_eq!(result, 4); // Length of "-inf"
    assert_eq!(str::from_utf8(&buffer[..result]).unwrap(), "-inf");
}

#[test]
fn test_write_to_ryu_buffer_nan() {
    let mut buffer: [u8; 20] = [0; 20];
    let result = unsafe { f32::NAN.write_to_ryu_buffer(buffer.as_mut_ptr()) };
    assert_eq!(result, 3); // Length of "NaN"
    assert_eq!(str::from_utf8(&buffer[..result]).unwrap(), "NaN");
}

#[test]
fn test_write_to_ryu_buffer_small_value() {
    let mut buffer: [u8; 20] = [0; 20];
    let result = unsafe { 1.23_f32.write_to_ryu_buffer(buffer.as_mut_ptr()) };
    assert_eq!(result, 4); // Length of "1.23"
    assert_eq!(str::from_utf8(&buffer[..result]).unwrap(), "1.23");
}

#[test]
fn test_write_to_ryu_buffer_large_value() {
    let mut buffer: [u8; 20] = [0; 20];
    let result = unsafe { 123456789.0_f32.write_to_ryu_buffer(buffer.as_mut_ptr()) };
    assert_eq!(result, 11); // Length of "123456789"
    assert_eq!(str::from_utf8(&buffer[..result]).unwrap(), "123456789");
}

