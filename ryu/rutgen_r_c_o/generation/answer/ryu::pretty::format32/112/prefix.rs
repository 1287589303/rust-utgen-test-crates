// Answer 0

#[test]
unsafe fn test_format32_case_zero() {
    let f: f32 = 0.0;
    let mut buffer = [0u8; 16];
    let result = buffer.as_mut_ptr();
    let len = format32(f, result);
}

#[test]
unsafe fn test_format32_case_negative_k() {
    let f: f32 = 0.0; // ieee_exponent == 0, ieee_mantissa == 0, sign is false
    let mut buffer = [0u8; 16];
    let result = buffer.as_mut_ptr();
    let len = format32(f, result);
}

#[test]
unsafe fn test_format32_case_kk_zero() {
    let f: f32 = 0.0; // kk == 0, sign is false
    let mut buffer = [0u8; 16];
    let result = buffer.as_mut_ptr();
    let len = format32(f, result);
}

