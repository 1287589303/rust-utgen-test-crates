// Answer 0

#[test]
fn test_shl_valid_case_0() {
    let a: usize = 1;
    let b: usize = 0;
    let what: &'static str = "test_case_0";
    let result = shl(a, b, what);
    let expected = Ok(1);
    assert_eq!(result, expected);
}

#[test]
fn test_shl_valid_case_1() {
    let a: usize = 2;
    let b: usize = 1;
    let what: &'static str = "test_case_1";
    let result = shl(a, b, what);
    let expected = Ok(4);
    assert_eq!(result, expected);
}

#[test]
fn test_shl_valid_case_2() {
    let a: usize = 3;
    let b: usize = 2;
    let what: &'static str = "test_case_2";
    let result = shl(a, b, what);
    let expected = Ok(12);
    assert_eq!(result, expected);
}

#[test]
fn test_shl_valid_case_3() {
    let a: usize = usize::MAX >> 1; // Ensures enough room for shifting
    let b: usize = 1;
    let what: &'static str = "test_case_3";
    let result = shl(a, b, what);
    let expected = Ok(usize::MAX);
    assert_eq!(result, expected);
}

#[test]
fn test_shl_valid_case_4() {
    let a: usize = 15;
    let b: usize = 3;
    let what: &'static str = "test_case_4";
    let result = shl(a, b, what);
    let expected = Ok(120);
    assert_eq!(result, expected);
}

