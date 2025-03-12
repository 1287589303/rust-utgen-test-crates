// Answer 0

#[test]
fn test_d2d_case1() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // Exponent is zero, which satisfies the precondition

    // Call the d2d function
    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, -1022); // Expecting a specific exponent based on initialization logic
    assert_eq!(result.mantissa, 1); // The expected mantissa after conversion
}

#[test]
fn test_d2d_case2() {
    let ieee_mantissa: u64 = 1000; // Non-zero mantissa
    let ieee_exponent: u32 = 2048; // Non-zero exponent to meet the condition for e2 < 0

    // Call the d2d function
    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, 0); // Expecting a specific exponent based on the computations within d2d
    assert_eq!(result.mantissa, 2000); // Expected resulting mantissa after computation
}

#[test]
fn test_d2d_case3() {
    let ieee_mantissa: u64 = 512; // Non-zero mantissa
    let ieee_exponent: u32 = 1021; // Exponent set so that e2 remains >= 0

    // Call the d2d function
    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, -1); // Check for a valid exponent computation
    assert_eq!(result.mantissa, 1024); // Expected behavior based on computation
}

#[test]
fn test_d2d_case4() {
    let ieee_mantissa: u64 = 2500; // Non-zero mantissa
    let ieee_exponent: u32 = 1024; // Mixed exponent

    // Call the d2d function
    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, 1); // Checking expected exponent value
    assert_eq!(result.mantissa, 6250); // Expected computed mantissa
}

#[test]
fn test_d2d_case5() {
    let ieee_mantissa: u64 = 560; // Non-zero mantissa
    let ieee_exponent: u32 = 1025; // Exponent ensuring conditions are met

    // Call the d2d function
    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, 2); // Specific exponent expectation
    assert_eq!(result.mantissa, 4480); // Check the computed mantissa
}

