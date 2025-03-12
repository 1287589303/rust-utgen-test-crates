// Answer 0

#[test]
fn test_digits_with_under_scores() {
    let input = Cursor {
        rest: "___",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

#[test]
fn test_digits_with_leading_underscore() {
    let input = Cursor {
        rest: "_validInput",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

#[test]
fn test_digits_with_multiple_underscores() {
    let input = Cursor {
        rest: "a__b__c",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

#[test]
fn test_digits_with_two_leading_underscores() {
    let input = Cursor {
        rest: "__noDigitsHere",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

#[test]
fn test_digits_with_trailing_underscores() {
    let input = Cursor {
        rest: "validInput____",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = digits(input);
}

