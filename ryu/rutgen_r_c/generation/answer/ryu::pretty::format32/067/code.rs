// Answer 0

#[test]
fn test_format32_case1() {
    let f: f32 = 1.0; // sign is false, ieee_exponent is 0 (false), ieee_mantissa is 0 (true)
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let len = format32(f, result_ptr);
        let output = core::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(output, "1e0"); 
    }
}

#[test]
fn test_format32_case2() {
    let f: f32 = 1000000.0; // sign is false, ieee_exponent is not 0, ieee_mantissa is 0
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let len = format32(f, result_ptr);
        let output = core::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(output, "1e6"); 
    }
}

#[test]
fn test_format32_case3() {
    let f: f32 = 123456789.0; // sign is false, ieee_exponent is not 0, ieee_mantissa is 0
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let len = format32(f, result_ptr);
        let output = core::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(output, "1.23456789e8"); 
    }
}

#[test]
fn test_format32_case4() {
    let f: f32 = 0.0000001; // sign is false, ieee_exponent is not 0, ieee_mantissa is 0
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let len = format32(f, result_ptr);
        let output = core::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(output, "1e-7"); 
    }
}

#[test]
fn test_format32_case5() {
    let f: f32 = 1e-40; // valid number, sign is false, ieee_exponent is not 0, ieee_mantissa is 0
    let mut buffer = [0u8; 16];
    let result_ptr = buffer.as_mut_ptr();
    unsafe {
        let len = format32(f, result_ptr);
        let output = core::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(output, "1e-40"); 
    }
}

