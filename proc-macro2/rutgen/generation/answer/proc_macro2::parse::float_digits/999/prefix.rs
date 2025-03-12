// Answer 0

#[test]
fn test_float_digits_with_invalid_after_dot() {
    let input = Cursor {
        rest: "0..",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = float_digits(input);
}

#[test]
fn test_float_digits_with_invalid_after_dot_identifier() {
    let input = Cursor {
        rest: "0.a",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = float_digits(input);
}

