// Answer 0

#[test]
fn test_digits_with_leading_underscores() {
    let input = Cursor {
        rest: "____",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

#[test]
fn test_digits_with_single_leading_underscore() {
    let input = Cursor {
        rest: "_",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

#[test]
fn test_digits_with_multiple_underscores() {
    let input = Cursor {
        rest: "____",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

#[test]
fn test_digits_with_empty_input() {
    let input = Cursor {
        rest: "",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

