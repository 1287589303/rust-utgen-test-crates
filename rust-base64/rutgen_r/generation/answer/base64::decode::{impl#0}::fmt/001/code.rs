// Answer 0

#[test]
fn test_invalid_padding_fmt() {
    use std::fmt;

    #[derive(Debug)]
    enum DecodeError {
        InvalidPadding,
    }

    impl fmt::Display for DecodeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Self::InvalidPadding => write!(f, "Invalid padding"),
            }
        }
    }

    let error = DecodeError::InvalidPadding;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid padding");
}

