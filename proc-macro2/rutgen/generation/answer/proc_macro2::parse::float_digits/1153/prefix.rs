// Answer 0

#[test]
fn test_float_digits_valid_case_1() {
    let input = Cursor { rest: "1.0", off: 0 };
    float_digits(input);
}

#[test]
fn test_float_digits_valid_case_2() {
    let input = Cursor { rest: "1e10", off: 0 };
    float_digits(input);
}

#[test]
fn test_float_digits_valid_case_3() {
    let input = Cursor { rest: "1E-10", off: 0 };
    float_digits(input);
}

#[test]
fn test_float_digits_valid_case_4() {
    let input = Cursor { rest: "1.0e+1", off: 0 };
    float_digits(input);
}

#[test]
fn test_float_digits_valid_case_5() {
    let input = Cursor { rest: "1_0.0", off: 0 };
    float_digits(input);
}

#[test]
fn test_float_digits_valid_case_6() {
    let input = Cursor { rest: "1_0e-1", off: 0 };
    float_digits(input);
}

#[test]
fn test_float_digits_invalid_case_1() {
    let input = Cursor { rest: "01.0.1", off: 0 };
    float_digits(input);
}

#[test]
fn test_float_digits_invalid_case_2() {
    let input = Cursor { rest: "1e1.0", off: 0 };
    float_digits(input);
}

