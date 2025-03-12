// Answer 0

#[test]
fn test_float_digits_case1() {
    let cursor = Cursor { rest: "1.2e+3" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_case2() {
    let cursor = Cursor { rest: "0.123" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_case3() {
    let cursor = Cursor { rest: "2.3e3" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_case4() {
    let cursor = Cursor { rest: "10_000" };
    let _ = float_digits(cursor);
}

#[test]
fn test_float_digits_case5() {
    let cursor = Cursor { rest: "10e-5" };
    let _ = float_digits(cursor);
}

