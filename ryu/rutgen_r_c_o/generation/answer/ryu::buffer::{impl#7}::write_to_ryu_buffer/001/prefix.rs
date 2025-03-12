// Answer 0

#[test]
fn test_write_to_ryu_buffer_normal() {
    let value: f64 = 123.456;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_zero() {
    let value: f64 = 0.0;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_negative_zero() {
    let value: f64 = -0.0;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_subnormal() {
    let value: f64 = 1e-300;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_infinity() {
    let value: f64 = f64::INFINITY;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_negative_infinity() {
    let value: f64 = f64::NEG_INFINITY;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_nan() {
    let value: f64 = f64::NAN;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_max() {
    let value: f64 = 1.7976931348623157e308;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

#[test]
fn test_write_to_ryu_buffer_min() {
    let value: f64 = -1.7976931348623157e308;
    let mut result: [u8; 32] = unsafe { MaybeUninit::uninit().assume_init() };
    let size = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
}

