// Answer 0

#[test]
fn test_write_to_ryu_buffer_zero() {
    let mut buffer: [MaybeUninit<u8>; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let f: f64 = 0.0;
    let len = unsafe { f.write_to_ryu_buffer(result) };
    
    let output: &[u8] = unsafe { slice::from_raw_parts(result, len) };
    assert_eq!(str::from_utf8(output).unwrap(), "0.0");
}

#[test]
fn test_write_to_ryu_buffer_nan() {
    let mut buffer: [MaybeUninit<u8>; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let f: f64 = f64::NAN;
    let len = unsafe { f.write_to_ryu_buffer(result) };
    
    let output: &[u8] = unsafe { slice::from_raw_parts(result, len) };
    assert_eq!(str::from_utf8(output).unwrap(), "NaN");
}

#[test]
fn test_write_to_ryu_buffer_infinity() {
    let mut buffer: [MaybeUninit<u8>; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let f: f64 = f64::INFINITY;
    let len = unsafe { f.write_to_ryu_buffer(result) };
    
    let output: &[u8] = unsafe { slice::from_raw_parts(result, len) };
    assert_eq!(str::from_utf8(output).unwrap(), "inf");
}

#[test]
fn test_write_to_ryu_buffer_negative_infinity() {
    let mut buffer: [MaybeUninit<u8>; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = unsafe { buffer.as_mut_ptr() };
    let f: f64 = f64::NEG_INFINITY;
    let len = unsafe { f.write_to_ryu_buffer(result) };
    
    let output: &[u8] = unsafe { slice::from_raw_parts(result, len) };
    assert_eq!(str::from_utf8(output).unwrap(), "-inf");
}

