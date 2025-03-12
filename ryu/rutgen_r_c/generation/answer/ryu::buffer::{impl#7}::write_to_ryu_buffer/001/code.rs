// Answer 0

#[test]
fn test_write_to_ryu_buffer_zero() {
    let mut buffer: [MaybeUninit<u8>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let size = unsafe { (0.0_f64).write_to_ryu_buffer(result) };
    let output = unsafe { core::str::from_utf8(&buffer[..size]).unwrap() };
    assert_eq!(output, "0.0");
}

#[test]
fn test_write_to_ryu_buffer_positive_infinity() {
    let mut buffer: [MaybeUninit<u8>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let size = unsafe { (f64::INFINITY).write_to_ryu_buffer(result) };
    let output = unsafe { core::str::from_utf8(&buffer[..size]).unwrap() };
    assert_eq!(output, "inf");
}

#[test]
fn test_write_to_ryu_buffer_negative_infinity() {
    let mut buffer: [MaybeUninit<u8>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let size = unsafe { (-f64::INFINITY).write_to_ryu_buffer(result) };
    let output = unsafe { core::str::from_utf8(&buffer[..size]).unwrap() };
    assert_eq!(output, "-inf");
}

#[test]
fn test_write_to_ryu_buffer_nan() {
    let mut buffer: [MaybeUninit<u8>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let size = unsafe { (f64::NAN).write_to_ryu_buffer(result) };
    let output = unsafe { core::str::from_utf8(&buffer[..size]).unwrap() };
    assert_eq!(output, "NaN");
}

#[test]
fn test_write_to_ryu_buffer_small_positive() {
    let mut buffer: [MaybeUninit<u8>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let size = unsafe { (0.001_f64).write_to_ryu_buffer(result) };
    let output = unsafe { core::str::from_utf8(&buffer[..size]).unwrap() };
    assert_eq!(output, "0.001");
}

#[test]
fn test_write_to_ryu_buffer_large_number() {
    let mut buffer: [MaybeUninit<u8>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let size = unsafe { (1234567890.0_f64).write_to_ryu_buffer(result) };
    let output = unsafe { core::str::from_utf8(&buffer[..size]).unwrap() };
    assert_eq!(output, "1234567890");
}

