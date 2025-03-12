// Answer 0

#[test]
fn test_shl_overflow_case_1() {
    let a: usize = 1; // a is small enough
    let b: usize = 32; // b is at the threshold of causing overflow
    let what: &'static str = "test";
    let result = shl(a, b, what);
}

#[test]
fn test_shl_overflow_case_2() {
    let a: usize = usize::MAX >> 1; // a is at its maximum without overflow
    let b: usize = 32; // b is at the threshold of causing overflow
    let what: &'static str = "test";
    let result = shl(a, b, what);
}

#[test]
fn test_shl_overflow_case_3() {
    let a: usize = usize::MAX >> 1; // a is at its maximum
    let b: usize = 64; // b definitely causes overflow
    let what: &'static str = "test";
    let result = shl(a, b, what);
}

