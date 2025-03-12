// Answer 0

#[test]
fn test_d2d_zero_exponent() {
    let ieee_mantissa = 0u64;
    let ieee_exponent = 0u32;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 0);
    assert_eq!(result.exponent, 1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2);
}

#[test]
fn test_d2d_one() {
    let ieee_mantissa = 1u64;
    let ieee_exponent = 1023u32; // represents 1.0 in IEEE 754
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 1);
    assert_eq!(result.exponent, 1); // 1.0 represents 1 * 10^0
}

#[test]
fn test_d2d_small_number() {
    let ieee_mantissa = 1u64;
    let ieee_exponent = 1022u32; // represents 0.5 in IEEE 754
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 5);
    assert_eq!(result.exponent, -1); // 0.5 represents 5 * 10^{-1}
}

#[test]
fn test_d2d_large_number() {
    let ieee_mantissa = 0xFFFFFFFFFFFFFu64; // largest mantissa
    let ieee_exponent = 2047u32; // represents infinity in IEEE 754
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 0); // General case, as it's not a valid finite number
    assert_eq!(result.exponent, 0); // Exponent default if not handled
}

#[test]
fn test_d2d_even_mantissa() {
    let ieee_mantissa = 2u64;
    let ieee_exponent = 1024u32; // represents 2.0 in IEEE 754
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 2);
    assert_eq!(result.exponent, 1); // Exponent represents 2 * 10^0
}

#[test]
fn test_d2d_odd_mantissa() {
    let ieee_mantissa = 3u64;
    let ieee_exponent = 1025u32; // represents 3.0 in IEEE 754
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 3);
    assert_eq!(result.exponent, 1); // Exponent represents 3 * 10^0
}

#[test]
fn test_d2d_negative_exponent() {
    let ieee_mantissa = 5u64;
    let ieee_exponent = 1021u32; // represents 0.25 in IEEE 754
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 25);
    assert_eq!(result.exponent, -2); // 0.25 represents 25 * 10^{-2}
}

