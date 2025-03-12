// Answer 0

#[derive(Debug)]
enum StrSimError {
    DifferentLengthArgs,
}

use std::fmt::{self, Formatter};

impl fmt::Display for StrSimError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match self {
            StrSimError::DifferentLengthArgs => "Differing length arguments provided",
        };

        write!(fmt, "{}", text)
    }
}

#[test]
fn test_fmt_different_length_args() {
    let error = StrSimError::DifferentLengthArgs;
    let result = format!("{}", error);
    assert_eq!(result, "Differing length arguments provided");
}

